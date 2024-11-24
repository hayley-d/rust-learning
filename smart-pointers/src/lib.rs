// refCell enforces borrowing rules at run time not compile time

mod reff_cell {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T: Messenger> LimitTracker<'a, T> {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<T> {
            return LimitTracker {
                messenger,
                value: 0,
                max,
            };
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percent_of_max: f64 = self.value as f64 / self.max as f64;

            if percent_of_max >= 1.0 {
                self.messenger
                    .send("Warning: You have used over 100% of the capacity");
            } else if percent_of_max >= 0.9 {
                self.messenger
                    .send("Warning: You have used over 90% of the capacity");
            } else if percent_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You have used over 75% of the capacity");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use self::reff_cell::*;
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            return MockMessenger {
                sent_messages: RefCell::new(Vec::new()),
            };
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn messenger_test() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // dont need a mutable reference here
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
