use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn a() {
    println!("Hello world");
}

fn b() {
    a();
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    c(numbers.iter().sum());
}

fn c(num: i32) {
    if num < 100 {
        panic!("Number is less than 100");
    }
}

fn main() {
    // This will crash the program
    b();
    let f: Result<File, Error> = File::open("Hello.txt");
    let my_file: File = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(new_file) => new_file,
                Err(err) => panic!("Could not create new file: {}", err),
            },
            _ => panic!("Some other type of error"),
        },
    };

    let res: Result<String, Error> = read_username_from_file();
    match res {
        Ok(username) => println!("The username is {}", username),
        Err(e) => panic!("Error occured: {:?}", e),
    }
}

// The Ruesult enum
/*enum Result<T, E> {
    Ok(T),
    Err(E),
}*/

/**
* This function is more simple
*/

/**
* Function has been simplified further.
* The ? will cause the function to return an error streigh away without continuing with the
* function logic.
*/
fn read_username_from_file_v4() -> Result<String, Error> {
    return fs::read_to_string("Username.txt");
}

fn read_username_from_file_v3() -> Result<String, Error> {
    let mut username: String = String::new();
    File::open("Username.txt")?.read_to_string(&mut username)?;
    return Ok(username);
}

fn read_username_from_file_v2() -> Result<String, Error> {
    let mut file_result/*: Result<File, Error>*/ = File::open("Username.txt")?;

    let mut username: String = String::new();
    file_result.read_to_string(&mut username)?;
    return Ok(username);
}

fn read_username_from_file() -> Result<String, Error> {
    let file_result: Result<File, Error> = File::open("Username.txt");

    let mut my_file = match file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username: String = String::new();

    match my_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
