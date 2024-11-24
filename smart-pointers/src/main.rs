use std::rc::Rc;

// Initially this will cause an error as the compiler needs to know the size at compile time but
// because this is recursive it is impossible to know the size.
// By adding Box pointer to encapulate the list it allows the memory to be allocated to the heap.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct BSNode<'a> {
    left: &'a BSNode<'a>,
    right: &'a BSNode<'a>,
    value: i32,
}

struct Node {
    left: Box<Node>,
    right: Box<Node>,
    value: i32,
}

use std::fmt::Display;
use std::ops::Deref;

use List::{Cons, Nil};

fn main() {
    // allows you to allocate memory on the heap
    // no overhead besides storing a value on the heap
    // not a fat pointer
    let list: Rc<List> = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("The count after all this is {}", Rc::strong_count(&list)); // count of 1

    let a: List = Cons(3, Rc::clone(&list));
    println!("The count after all this is {}", Rc::strong_count(&list)); // count of 2

    let b: List = Cons(3, Rc::clone(&list));
    println!("The count after all this is {}", Rc::strong_count(&list)); // count of 3

    // clone does not create a deep copy it increments the reference count of the fat pointer
}

// The defef trait allows you to derefrence the pointers

fn foo() {
    let x: i32 = 5;
    let y: &i32 = &x;
    // y is a pointer to x
    // you need to derefrence y to get the value pointed to by y
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, *y);

    // now y points to a copy of x because ints are copied when ownership

    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let name: MyBox<String> = MyBox::new(String::from("Hayley"));

    say_hi(&name);
    // &MyBox<String> -> &String basically equivalent to &str if deref is called
}

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping the pointer with data {}", self.0);
    }
}

fn say_hi(name: &str) {
    let 🦀 = '🦀';
    println!("Hello, {}!", name);
}
