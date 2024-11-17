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
