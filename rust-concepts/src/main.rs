#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
    online: bool,
}

impl Rectangle {
    fn area_of_rectangle(&self) -> f32 {
        return self.width * self.height;
    }

    pub fn can_contain(&self, other: &Rectangle) -> bool {
        return self.area_of_rectangle() >= other.area_of_rectangle();
    }
}

impl Rectangle {
    fn square(size: f32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}

fn main() {
    let mut user1: User = User {
        username: String::from("Hayley"),
        email: String::from("hayleydod@proton.me"),
        age: 22,
        online: true,
    };

    let username: String = user1.username;
    user1.username = String::from("haylo");

    let user2: User = user_factory(String::from("Sloth"), String::from("email@gamil.com"), 8);

    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);
    let rec: Rectangle = Rectangle {
        width: 30.5,
        height: 2.5,
    };

    println!("{:#?}", user1);
    println!("{}", rec.area_of_rectangle());

    let square: Rectangle = Rectangle::square(7.5);
    println!(
        "The widht and height of the square is {}cm in length",
        square.width
    );
}

fn user_factory(username: String, email: String, age: u32) -> User {
    return User {
        username,
        email,
        age,
        online: false,
    };
}

struct Rectangle {
    width: f32,
    height: f32,
}

/*fn area_of_rectangle(rec: Rectangle) -> f32 {
    return rec.width * rec.height;
}*/
fn main2() {
    // A 32-bit signed integer (immutable)
    let x: i32 = 7;
    println!("The value of x is {}", x);

    // Shadowing x
    let x: &str = "Seven";
    println!("The value of x is {}", x);

    // Constatnts vs immutable
    // Constants need a type declaration the type cannot be inferred like variaables
    // Constants cannot be shadowed even though a variable is immutable it can still be shadowed
    const MY_CONSTANT: u8 = 1;
    println!("The value of my constatant is {}", MY_CONSTANT);

    let _y = "Hello"; // This type is inferred as a &str

    // Mutable variables value can change
    let mut z: i32 = 2;
    println!("The value of z is {}", z);
    z = 9;
    println!("The value of z is {}", z);

    // Scalar Data types

    // Integers
    let _num1: i8 = 25;
    let _num2: i16 = 1;
    let _num3: i32 = 3;
    let _num4: i64 = 4;
    let _num5: i128 = 4;

    // Only positive
    let _unisgned_num1: u8 = 1;
    let _unisgned_num2: u16 = 1;
    let _unisgned_num3: u32 = 3;
    let _unisgned_num4: u64 = 4;
    let _unisgned_num5: u128 = 4;

    // Floating point numbers
    let _float1: f32 = 7.0;
    let _flaot2: f64 = 7.55;

    // Booleans
    let _truthy: bool = true;
    let _falsey: bool = false;

    // Character
    let _char1: char = 'a';
    let _char2: char = '4';

    // Tuple
    let my_tup: (&str, i32) = ("Ferris is great", 7);
    let (sentance, luck_num) = my_tup;
    println!("The sentance is {}", sentance);
    println!("Lucky number is {}", luck_num);

    println!("The sentance is {}", my_tup.0);
    println!("Lucky number is {}", my_tup.1);

    // Arrays
    // This array has fixed length by the _
    let my_arr: [i32; _] = [1, 2, 3, 4];
    // This array has a length of 4
    let my_arr2: [i32; 4] = [1, 2, 3, 4];
    foo();
    fow(7, "Ferris is cute");

    let sum: i32 = sum(7, 5);

    //  Control flow
    if sum > 4 {
        println!("Greater than 4");
    } else {
        println!("Less than 4");
    }

    // Declare a variable the value of an if statement
    let condition: bool = true;

    let _my_num4: i32 = if condition { 7 } else { 27 };

    // Loop
    let mut counter: i32 = 0;
    loop {
        println!("The loop interation is at {}", counter);
        counter += 1;
        if counter > 3 {
            break;
        }
    }
}

fn foo() {
    println!("Foo");
}

fn fow(x: i32, y: &str) {
    println!("The string is {} and the number is {}", y, x);
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn count_to_ten() {
    let mut counter: i32 = 0;
    let count: i32 = loop {
        counter += 1;
        if counter >= 10 {
            break counter;
        }
    };

    println!("Counted to {}", count);
}

fn while_looping() {
    let mut counter: i32 = 10;
    while counter != 0 {
        counter -= 1;
    }
}

fn for_looping() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    for element in numbers.iter() {
        println!("{}", element);
    }

    // Prints for every number in the given rage take the bumber as the element and do something
    // (last number is exclusive)
    for number in 1..5 {
        println!("{}", number);
    }
}

fn only_odd(x: i32) -> bool {
    if x % 2 == 0 {
        unreachable!("This should never happen");
    }
    return true;
}

fn only_evens(x: i32) -> bool {
    todo!("Finish later");
}
