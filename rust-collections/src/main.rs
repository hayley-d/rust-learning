use std::collections::HashMap;

fn hashmaps() {
    let mut blue: String = String::from("Blue");
    let red: String = String::from("Red");

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(red, 11);

    let blue_score: u32 = match scores.get("Blue") {
        Some(i) => *i,
        None => 0,
    };

    println!("The score of the blue team is {}", blue_score);

    for (key, value) in &scores {
        println!("Team {} has a score of {}", key, value);
    }

    scores.entry(String::from("Red")).or_insert(30);

    let s = scores.entry("Yellow".to_string()).or_insert(0);
    // you get a &mut so you can change the value in the hashmap
    *s = 10;
}

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

fn strings() {
    let mut s1: String = String::from("Hello world");
    s1.push_str("!");

    let s2: String = "hello worlds".to_string();

    let s3: String = s1 + &s2;

    let c: char = if let Some(c) = s1.chars().get(0) {
        c
    } else {
        '\0'
    };
}
