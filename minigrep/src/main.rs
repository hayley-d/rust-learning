use minigrep::{run, Query};
use std::env;
use std::process;

fn main() {
    // args returns an iterator over the arguemnts passed to the program through the command line
    let args: Vec<String> = env::args().collect();

    let query: Query = match Query::new(&args) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run(query) {
        eprintln!("Appliction Error: {}", e);
        process::exit(1);
    }
}
