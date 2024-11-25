use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use submarine::submarine::*;

fn main() -> std::io::Result<()> {
    let mut my_sub: Submarine = Submarine::new();
    let mut my_file: File = match File::open("direction.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error occured when attempting to open the file: {:?}", e);
            return Err(e);
        }
    };

    let reader = BufReader::new(my_file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        let (direction, units): (&str, &str) = match line.split_once(' ') {
            Some(t) => t,
            None => {
                eprintln!("Error splitting string");
                continue;
            }
        };

        match direction {
            "forward" => my_sub.forward(units.parse::<isize>().unwrap()),
            "backward" => my_sub.backward(units.parse::<isize>().unwrap()),
            "up" => my_sub.up(units.parse::<isize>().unwrap()),
            _ => my_sub.down(units.parse::<isize>().unwrap()),
        }
    }

    println!(
        "The submarine is at location: x:{};y:{}",
        my_sub.x, my_sub.y
    );

    assert_eq!(15, my_sub.x);
    assert_eq!(10, my_sub.y);
    return Ok(());
}
