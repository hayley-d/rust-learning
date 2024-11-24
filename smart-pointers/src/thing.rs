use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

mod ref_cycles {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    pub fn use_list() {
        // this contains a memory leak, as there is a circular dependancy between a and b
        // a references b and b references a so at runtime this will cause a stack overflow
        let a: Rc<List> = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b: Rc<List> = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("b rc count after link creation = {}", Rc::strong_count(&b));
        println!("a rc count after link creation = {}", Rc::strong_count(&a));
    }
}
