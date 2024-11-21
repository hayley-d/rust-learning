use std::env;
use std::fs;

fn main() {
    // args returns an iterator over the arguemnts passed to the program through the command line
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);

    let query: &str = match &args.get(1) {
        Some(v) => v,
        None => "",
    };

    let file_name: &str = match &args.get(2) {
        Some(v) => v,
        None => "",
    };

    println!("Searching for {}", query);
    println!("In file {}", file_name);

    let contents: String = match fs::read_to_string(&file_name) {
        Ok(s) => s,
        Err(e) => panic!("Something went wrong when attempting to read the file"),
    };
}
