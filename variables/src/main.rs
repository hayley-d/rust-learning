fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is now {x}");

    let y = 5;

    let y = y + 2;

    {
        let y = y * 2;
        println!("The value of y is {y}");
    }

    println!("The value of y is now {y}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("There were {spaces} spaces");

    let z: f32 = 3.05;
    println!("My float is {z}");

    //sum
    let _sum: u32 = 5 + 2;

    //subtraction
    let _sub: u32 = 9 - 2;

    //divison
    let _div: u32 = 14 / 2;

    let _mul: i32 = (-1) * 7;

    let _rem: u32 = 40 % 2;

    let _boolean_type: bool = true;

    let _unused_char: char = 'H';

    let _my_tuple: (u32, char, bool) = (7, 'h', true);

    const TUPLE_USE: (bool, char, f32) = (true, 'H', 7.54);

    let (h, j, k) = TUPLE_USE;
    println!("The first tuple element is {h}");
    println!("The second tuple element is {j}");
    println!("The third tuple element is {k}");

    println!("The first tuple element is {}", TUPLE_USE.0);
    println!("The second tuple element is {}", TUPLE_USE.1);
    println!("The third tuple element is {}", TUPLE_USE.2);

    let my_array: [char; 6] = ['H', 'a', 'y', 'l', 'e', 'y'];

    let mut counter = 0;
    loop {
        if counter < 6 {
            let element = my_array[counter];
            println!("{element}");
            counter += 1;
        } else {
            break;
        }
    }
}
