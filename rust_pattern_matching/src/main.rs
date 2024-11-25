fn main() {
    let (name, surname): (&str, &str) = ("Hayley", "Dodkins");
    println!("Hi, my name is {} {}", name, surname);

    let language: Language = Language::English;

    match language {
        Language::English => println!("I speak English"),
        Language::French => println!("I speak French"),
        Language::Spanish => println!("I speak Spanish"),
        Language::Danish => println!("I speak Danish"),
        Language::Japanese => println!("I speak Japanese"),
    }

    let auth_status: Option<&str> = None;
    let is_admin: bool = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = auth_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        println!("Authorization");
    }

    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v: Vec<char> = vec!['a', 'b', 'c', 'd'];

    for (index, c) in v.iter().enumerate() {
        println!("{} is at index {}", c, index);
    }
}

enum Language {
    English,
    French,
    Spanish,
    Danish,
    Japanese,
}
