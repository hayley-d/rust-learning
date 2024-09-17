use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_guess = rand::thread_rng().gen_range(1..=100); //inclusive on lower and upper bounds
    println!("The secret number was {secret_guess}");

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_guess) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You got it!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too Large"),
        }
    }
}
