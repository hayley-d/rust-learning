use std::fs::{self, File};
use std::io::{self, Error, Read};

fn main() {}

pub fn read_file(path: String) -> Result<String, Error> {
    let mut line: String = String::new();
    File::open(path)?.read_to_string(&mut line)?;
    return Ok(line);
}
