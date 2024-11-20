struct Rectangle {
    width: i32,
    height: i32,
}

struct Guess {
    value: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width * self.height > other.width * other.height;
    }
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The guess value should be between 1 and 100");
        }

        return Guess { value };
    }
}

fn main() {
    println!("7+8={}", add(7, 8));
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn double_value(x: u32) -> u32 {
    return x * 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let result = add(7, 7);
        assert_eq!(result, 14);
    }

    #[test]
    fn rectangle_can_hold_test() {
        let larger = Rectangle {
            width: 7,
            height: 1,
        };

        let smaller: Rectangle = Rectangle {
            width: 3,
            height: 1,
        };

        assert!(
            larger.can_hold(&smaller),
            "Larger rectangle could not hold the smaller rectangle"
        );
    }

    #[test]
    fn test_double() {
        let result = double_value(7);
        assert_eq!(result, 14);
        assert_ne!(result, 15);
    }

    #[test]
    #[should_panic]
    fn test_guess() {
        Guess::new(-1);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            return Ok(());
        }

        return Err(String::from("two plus two does not equal four"));
    }
}
