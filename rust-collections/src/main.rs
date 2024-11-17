enum SpredsheetCell {
    Int(i32),
    Float(f32),
    Text(String),
}

// Collections are allocated on the heap
fn main() {
    let mut vector_of_ints: Vec<i32> = Vec::new();

    for i in 0..10 {
        vector_of_ints.push(i);
    }

    let mut vector_of_strings: Vec<&str> = vec!["Sloths", "are", "the", "best"];

    // get is good practice since it returns an option and prevents the case of index out of bounds
    match vector_of_strings.get(0) {
        Some(s) => println!("{}", s),
        None => println!("Nothing was found!"),
    };

    vector_of_strings.push("love");

    let row: Vec<SpredsheetCell> = vec![
        SpredsheetCell::Int(7),
        SpredsheetCell::Text(String::from("Hello")),
    ];

    match row.get(0) {
        Some(r) => match r {
            SpredsheetCell::Int(i) => println!("Is a number"),
            _ => println!("Not a number"),
        },
        None => println!("Not found"),
    }
}
