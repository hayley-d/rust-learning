use std::env;
use std::fs;
use std::io::{self, Write};

/// This is a CGI program that generates and manipulates a sequence of Fibonacci numbers.
/// It responds to two actions (`next` and `prev`) provided as query parameters:
/// - `next`: Computes the next set of Fibonacci numbers.
/// - `prev`: Computes the previous set of Fibonacci numbers.
///
/// The results are displayed in an HTML format, with links for navigation.
fn main() {
    // HTTP header
    println!("Content-Type: text/html\n");

    // Pares the query_string enviroment variable
    let query: String = match env::var("QUERY_STRING") {
        Ok(q) => q,
        Err(e) => panic!("Could not parse query stirng: {:?}", e),
    };

    let action: &str = if query.contains("prev") {
        "prev"
    } else {
        "next"
    };

    // file to store the fibonacci numbers
    let file_path: &str = "fib.txt";
    // if the file is not found default numbers to 0 1 1 the start of the sequence
    let contents: String = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => String::from("0 1 1"),
    };

    let mut numbers: Vec<usize> = contents
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    if numbers.len() != 3 {
        numbers = vec![0, 1, 1];
    }

    match action {
        "next" => {
            let next: usize = numbers[1] + numbers[2];
            numbers = vec![numbers[1], numbers[2], next];
        }
        _ => {
            let prev: usize = numbers[1] - numbers[0];
            numbers = vec![prev, numbers[0], numbers[1]];
        }
    }

    // write the updated numbers to the file
    let new_contents = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    match fs::write(file_path, new_contents) {
        Ok(()) => 1,
        Err(e) => panic!("Failed to write to file: {:?}", e),
    };

    // Generate HTML output
    println!("<html>");
    println!("<head><meta http-equiv=\"Cache-Control\" content=\"no-cache\"></head>");
    println!("<body>");
    println!("<h1>Fibonacci Sequence</h1>");
    println!("<p>{}, {}, {}</p>", numbers[0], numbers[1], numbers[2]);

    // Generate hyperlinks
    println!("<a href=\"/cgi-bin/fib?next\">Next</a>");
    if numbers[0] > 0 {
        println!(" | <a href=\"/cgi-bin/fib?prev\">Previous</a>");
    }
    println!("</body></html>");
}
