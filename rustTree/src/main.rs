use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    pub value: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub parent: Option<Weak<RefCell<Node>>>,
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

    pub fn set_parent(&mut self, parent: Weak<RefCell<Node>>) {
        self.parent = Some(parent);
    }

    pub fn set_left(&mut self, child: Option<Rc<RefCell<Node>>>) {
        self.left = child
    }

    pub fn set_right(&mut self, child: Option<Rc<RefCell<Node>>>) {
        self.right = child
    }

    pub fn get_parent(&mut self) -> Rc<RefCell<Node>> {
        return match &self.parent {
            Some(p) => match p.upgrade() {
                Some(p) => p,
                None => panic!("This node has no parent"),
            },
            None => panic!("This node has no parent"),
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

pub fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
    if child.borrow().value > parent.borrow().value {
        parent.borrow_mut().right = Some(Rc::clone(&child));
    } else {
        parent.borrow_mut().left = Some(Rc::clone(&child));
    }
    child.borrow_mut().set_parent(Rc::downgrade(parent));
}

fn main() {
    let root = Rc::new(RefCell::new(Node::new(7)));

    let first_leaf = Rc::new(RefCell::new(Node::new(1)));

    let second_leaf = Rc::new(RefCell::new(Node::new(8)));

    add_child(&Rc::clone(&root), Rc::clone(&first_leaf));
    println!("The parent is {:?}", *first_leaf.borrow_mut().get_parent());
    add_child(&Rc::clone(&root), Rc::clone(&second_leaf));
    println!("The parent is {:?}", *second_leaf.borrow_mut().get_parent());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nodes() {
        let root = Rc::new(RefCell::new(Node::new(7)));

        let first_leaf = Rc::new(RefCell::new(Node::new(1)));

        let second_leaf = Rc::new(RefCell::new(Node::new(8)));

        add_child(&Rc::clone(&root), Rc::clone(&first_leaf));
        add_child(&Rc::clone(&root), Rc::clone(&second_leaf));

        assert_eq!(1, first_leaf.borrow().value);
        assert_eq!(8, second_leaf.borrow().value);
    }
}
