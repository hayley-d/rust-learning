pub mod state {
    use std::fmt::Display;

    pub enum States {
        Empty,
        Pending,
        Approved,
        Published,
    }

    pub struct Post {
        pub state: States,
        pub content: String,
    }

    impl Post {
        pub fn new() -> Post {
            return Post {
                state: States::Empty,
                content: String::new(),
            };
        }

        pub fn add_content(&mut self, content: String) {
            self.content = content;
            self.state = States::Pending;
            println!("The state is now {}", self.state);
        }

        pub fn disapprove(&mut self) {
            self.state = States::Pending;
        }

        pub fn approve(&mut self) {
            match self.state {
                States::Pending => {
                    self.state = States::Approved;
                    println!("The state is now {}", self.state);
                }
                _ => println!("The post is not ready to be reviewed yet"),
            }
        }

        pub fn publish(&mut self) {
            match self.state {
                States::Approved => {
                    self.state = States::Published;
                    println!("The state is now {}", self.state);
                }
                _ => println!("The post is not ready to be published yet"),
            }
        }
    }

    impl States {
        pub fn update_state(&self) {
            match self {
                States::Empty => States::Pending,
                States::Pending => States::Approved,
                States::Approved => States::Published,
                States::Published => States::Published,
            };
            println!("The state is now {}", self);
        }
    }

    impl Display for States {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let _ = match self {
                States::Empty => write!(f, "Empty"),
                States::Pending => write!(f, "Pending"),
                States::Approved => write!(f, "Approved"),
                States::Published => write!(f, "Published"),
            };
            return Ok(());
        }
    }
}
