/**
* Module to represent the front of the house
* Contains two modules hosting and serving
*/
mod front_of_the_house {
    // child modules are by default private to the parent so front_of_house can on;y access hosting
    // if it is pub
    // Child modules can see what the parent defines
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist");
        }

        pub fn seat_at_table() {
            println!("Seating at a table");
            some_private_func();
        }

        fn some_private_func() {
            println!("Cannot directly call me from outside!");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Taking order");
        }

        pub fn server_order() {
            println!("Serving order");
        }

        pub fn take_payment() {
            println!("Taking payment");
        }
    }
}
