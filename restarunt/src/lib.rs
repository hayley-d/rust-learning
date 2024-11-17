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

mod back_of_house {
    fn serve_order() {
        println!("Serving the order");
    }

    pub mod kitchen {
        pub fn make_the_food() {
            println!("Making the food");
            // reference the parent module
            super::serve_order();
        }
    }
}

pub fn eat_at_restaurant() {
    // To use the module it needs to be public and contain public methods
    // Absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();
    // Relative path
    // Starts from the current module
    front_of_the_house::hosting::seat_at_table();

    crate::back_of_house::kitchen::make_the_food();
}

/*pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
