use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;

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

fn main() {
    // args returns an iterator over the arguemnts passed to the program through the command line
    let args: Vec<String> = env::args().collect();

    let query: Query = match Query::new(&args) {
        Ok(q) => q,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
}

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    let contents: String = read_file_to_string(&query.file_name)?;

    println!("The file contains: {}", &contents);
    return Ok(());
}

pub struct Query<'a> {
    pub query: &'a str,
    pub file_name: &'a str,
}

impl<'a> Query<'a> {
    fn new(args: &Vec<String>) -> Result<Query, MyError> {
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
