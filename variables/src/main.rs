fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is now {x}");

    let y = 5;

    let y = 7;

    {
        let y = y * 2;
        println!("The value of y is {y}");
    }

    println!("The value of y is now {y}");
}
