use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    pub value: i32,
    pub left: Option<RefCell<Rc<Node>>>,
    pub right: Option<RefCell<Rc<Node>>>,
    pub parent: Option<RefCell<Weak<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Node {
        return Node {
            value,
            left: None,
            right: None,
            parent: None,
        };
    }
}

impl Deref for Node {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.value;
    }
}

pub fn add_child(parent: &RefCell<Rc<Node>>, child: &RefCell<Rc<Node>>) {
    if child.borrow().value > parent.borrow().value {
        parent.borrow_mut().right = Some(RefCell::new(Rc::clone(&child.borrow())));
        child.borrow_mut().parent = Some(RefCell::new(Rc::downgrade(&parent.borrow())));
    } else {
        parent.borrow_mut().left = Some(RefCell::new(Rc::clone(&child.borrow())));
        child.borrow_mut().parent = Some(RefCell::new(Rc::downgrade(&parent.borrow())));
    }
}

fn main() {
    let root = Rc::new(Node::new(7));

    let first_leaf = Rc::new(Node::new(1));

    let second_leaf = Rc::new(Node::new(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes() {
        let root = Rc::new(Node::new(7));

        let first_leaf = Rc::new(Node::new(1));

        let second_leaf = Rc::new(Node::new(8));

        add_child(
            &RefCell::new(Rc::clone(&root)),
            &RefCell::new(Rc::clone(&first_leaf)),
        );

        add_child(
            &RefCell::new(Rc::clone(&root)),
            &RefCell::new(Rc::clone(&second_leaf)),
        );
    }
}
