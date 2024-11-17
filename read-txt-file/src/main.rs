use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("lines.txt");

    let mut file = match File::open(&path) {
        Ok(file) => file,
        _ => panic!("Could not file the file {}", path.display()),
    };

    let mut file_contents: String = String::new();

    match file.read_to_string(&mut file_contents) {
        Ok(_) => print!("{} contains:\n{}", path.display(), file_contents),
        _ => panic!("Could not read the file"),
    };
    //file goes out of scope and the file is closed
}

fn skip_two_take_two() {
    // I know unwrap is bad but this is trival for practice so leave me alone
    let file = std::fs::read_to_string("lines").unwrap();
    file.lines()
        .enumerate()
        .filter(|(index, content)| index % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| print!("{} ", line));
}

fn skip_two_lines() {
    // I know unwrap is bad but this is trival for practice so leave me alone
    let file = std::fs::read_to_string("lines").unwrap();
    file.lines()
        .enumerate()
        .filter(|(index, content)| index % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| print!("{} ", line));
}
