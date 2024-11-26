use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let mut count: usize = 0;

    let my_file = match fs::read_to_string("trees.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error occured when attempting to open the file: {:?}", e);
            return Err(e);
        }
    };

    let count: usize = my_file
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            return line.chars().nth(idx * 3 % line.len());
        })
        .filter(|&x| x == '#')
        .count();
    println!("The number of trees we hit were {}", count);

    return Ok(());
}
