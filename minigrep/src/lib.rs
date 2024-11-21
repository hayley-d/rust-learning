use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub enum MyError {
    InvalidFile,
    InvalidArgs,
}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::InvalidFile => write!(f, "An error occured while attempting to read the file"),
            MyError::InvalidArgs => {
                write!(f, "This fuction needs two arguments query and file name")
            }
        }
    }
}
pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    let contents: String = read_file_to_string(&query.file_name)?;

    //println!("The file contains: {}", &contents);

    for line in search(query.query, &contents) {
        println!("{}", line);
    }

    return Ok(());
}

pub struct Query<'a> {
    pub query: &'a str,
    pub file_name: &'a str,
}

impl<'a> Query<'a> {
    pub fn new(args: &Vec<String>) -> Result<Query, MyError> {
        let query: &str = match &args.get(1) {
            Some(v) => v,
            None => return Err(MyError::InvalidArgs),
        };

        let file_name: &str = match &args.get(2) {
            Some(v) => v,
            None => return Err(MyError::InvalidArgs),
        };

        println!("Searching for {}", query);
        println!("In file {}", file_name);

        return Ok(Query { query, file_name });
    }
}

pub fn read_file_to_string(file_name: &str) -> Result<String, MyError> {
    return match fs::read_to_string(&file_name) {
        Ok(s) => Ok(s),
        Err(_) => Err(MyError::InvalidFile),
    };
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();

    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let content: &str = "Rust\nsafe, fast, productive.\n Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rust";
        let content: &str = "Rust\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
