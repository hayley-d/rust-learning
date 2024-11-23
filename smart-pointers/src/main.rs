// Initially this will cause an error as the compiler needs to know the size at compile time but
// because this is recursive it is impossible to know the size.
// By adding Box pointer to encapulate the list it allows the memory to be allocated to the heap.
enum List {
    Cons(i32, Box<List>),
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

use List::{Cons, Nil};

fn main() {
    // allows you to allocate memory on the heap
    // no overhead besides storing a value on the heap
    // not a fat pointer
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
