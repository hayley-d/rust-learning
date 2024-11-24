use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    // move means that values are moved into the closure and the colsure becomes the owner of any
    // varaibles used
    let thread_1: JoinHandle<()> = thread::spawn(move || {
        for i in numbers.iter() {
            println!("Thread 1: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let thread_2: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 2: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Main Thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    match thread_1.join() {
        Ok(p) => p,
        Err(e) => eprintln!("There was an error with thread 1: {:?}", e),
    };
    match thread_2.join() {
        Ok(p) => p,
        Err(e) => eprintln!("There was an error with thread 2: {:?}", e),
    };
}
