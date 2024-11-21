use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::process;

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

    let contents: String = match read_file_to_string(&query.file_name) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
}

pub struct Query<'a> {
    pub query: &'a str,
    pub file_name: &'a str,
}

impl<'a> Query<'a> {
    fn new(args: &Vec<String>) -> Result<Query, Error> {
        let query: &str = match &args.get(1) {
            Some(v) => v,
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Not enough arguments provided",
                ))
            }
        };

        let file_name: &str = match &args.get(2) {
            Some(v) => v,
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Not enough arguments provided",
                ));
            }
        };

        println!("Searching for {}", query);
        println!("In file {}", file_name);

        return Ok(Query { query, file_name });
    }
}

pub fn read_file_to_string(file_name: &str) -> Result<String, Error> {
    return match fs::read_to_string(&file_name) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(
            ErrorKind::Other,
            "Failed to read contents of the file",
        )),
    };
}
