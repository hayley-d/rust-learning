fn main() {
    fn a() {
        // These are both stack allocated variables
        let x: &str = "Hello";
        let y: i32 = 22; // Copy not a move
        b();
    }

    fn b() {
        // Strings are heap allocated
        let x: String = String::from("world");
    }

    a();

    // ------ Ownership Rules ------
    // 1. Each value in Rust has a variable that is called its owner.
    // 2.There can olny be a single owner
    // 3. When the owner goes out of scope the value is destroyed.

    // S is not valid here as it is not in scope
    {
        let s: i32 = 7; // Now s is in scope
    }
    // S is no longer in scope and has been destroyed.
}

fn foo() {
    let x: i32 = 7;
    let y: i32 = x; // Copy of x

    let string_1: String = String::from("Hello");
    let string_2: String = string_1; // This is a move string_1 is invalidated and moved to
                                     // string_2
                                     //println!("{} wolrd!", string_1); // This would cause an error as assigning string_2 to string_1
                                     // moved the pointer to string 2 thus invalidating string 1;
    println!("{} wolrd!", string_2);

    let string_3: String = String::from("World");
    let string_4: String = string_3.clone(); // Now string_4 cloned string3 so string_3 is still
                                             // valid
}

fn func1() {
    let s: String = String::from("Hello world");
    takes_ownership(s);
    //    println!("{}", s); // this function no longer owns s so it can no longer be used
}
// s was MOVED into the some_string variable so no shallow copy
fn takes_ownership(some_string: String) {
    println!("{}", some_string); // This now owns the string
}

fn func2() {
    let x: i32 = 7;
    func3(x);
    println!("{}", x); // x can still be used as it is copied by default not moved
}

fn func3(x: i32) {
    println!("{}", x); // x is a copy of the original
}

fn func4() {
    let s1: String = gives_ownership();
    println!("{}", s1);
}

// Moves ownership to s1 variable
fn gives_ownership() -> String {
    let some_string: String = String::from("Hello World!");
    return some_string;
}

/*fn func5() {
    let s1: String = String::from("Hello World");
    let (s2, len) = calculate_length(s1);
}

fn calculate_length(s1: String) -> (String, usize) {
    let length: usize = s1.len();
    return (s, length);
}*/

fn func5() {
    let s1: String = String::from("Hello World");
    let len: usize = calculate_length(&s1);
}

fn calculate_length(s1: &String) -> usize {
    let length: usize = s1.len();
    return length;
}

fn func6() {
    let mut s1: String = String::from("Hello World");
    let len: usize = calculate_length_v2(&mut s1);
}

// Can only have a single mutable reference at a time but you can have multiple immutable
// references
// You cannot have a mutable reference if you have immutable references
fn calculate_length_v2(s1: &mut String) -> usize {
    let length: usize = s1.len();
    return length;
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn first_word_v2(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    let mut x: usize = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            x = i;
        }
    }

    return &s[..x];
}

fn interger_slice() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: &[i32] = &a[..1];
}
