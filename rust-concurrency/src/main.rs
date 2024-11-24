// multiple producer single consumer
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
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

    // create a channel
    let (sender, reciever): (Sender<String>, Receiver<String>) = mpsc::channel::<String>();
    // this clone gives us another sender linked to the same reciever
    let sender_2: Sender<String> = sender.clone();

    // cannot share the sender between threads so we need to use move here
    let thread_3 = thread::spawn(move || {
        let msg: String = String::from("Hi, I like sloths");

        match sender.send(msg) {
            Ok(m) => m,
            Err(e) => eprintln!("Error: {:?}", e),
        };

        let msgs = vec![
            String::from("The"),
            String::from("Grinch"),
            String::from("is"),
            String::from("one of the best christmas films"),
        ];

        for m in msgs {
            match sender.send(m) {
                Ok(m) => m,
                Err(e) => eprintln!("Error: {:?}", e),
            };
            thread::sleep(Duration::from_millis(5));
        }
    });

    let thread_4 = thread::spawn(move || {
        let msg: String = String::from("Hi, I like pugs");

        match sender_2.send(msg) {
            Ok(m) => m,
            Err(e) => eprintln!("Error: {:?}", e),
        };

        let msgs = vec![
            String::from("rust"),
            String::from("c++"),
            String::from("go"),
            String::from("zig"),
        ];

        for m in msgs {
            match sender_2.send(m) {
                Ok(m) => m,
                Err(e) => eprintln!("Error: {:?}", e),
            };
            thread::sleep(Duration::from_millis(3));
        }
    });

    // recv blocks the current threads execution (so here we are in the main thread) until a
    // message has been recieved
    //
    // try_recv does not block if there is a message it will return otherwise it returs an err()
    let try_recv = match reciever.try_recv() {
        Ok(_) => true,
        Err(_) => false,
    };

    /*while try_recv {
        let received_msg = match reciever.recv() {
            Ok(msg) => msg,
            Err(e) => panic!("Error: {:?}", e),
        };

        println!("Message Recieved: {}", received_msg);

        let try_recv = match reciever.try_recv() {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    match thread_3.join() {
        Ok(p) => p,
        Err(e) => eprintln!("There was an error with thread 2: {:?}", e),
    };*/

    for msg in reciever {
        println!("Message Recieved: {}", msg);
    }

    let my_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mut my_theads: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..10 {
        let mutex = Arc::clone(&my_mutex);
        my_theads.push(thread::spawn(move || {
            let mut num = mutex.lock().unwrap();
            *num += 1;
        }));
    }

    for thread in my_theads {
        thread.join().unwrap();
    }

    let mutex = Arc::clone(&my_mutex);

    println!("The value in the mutex is {:?}", mutex);
}
