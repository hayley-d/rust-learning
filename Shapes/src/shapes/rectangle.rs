use std::fmt::Display;

use crate::shapes::area::Area;

pub struct Rectangle<'a> {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub next: Option<&'a Rectangle<'a>>,
}

/**
* Implementation for the Rectangle struct.
* Implements contains_point for the Collides trait.
*/
impl<'a> Rectangle<'a> {
    pub fn set_next(&mut self, other: Option<&'a Rectangle<'a>>) {
        self.next = other;
    }

    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && self.x + self.width >= x && self.y <= y && self.y + self.height >= y;
    }
}

impl<'a> Area for Rectangle<'a> {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl<'a> Display for Rectangle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Rectangle({}x{})", self.width, self.height);
    }
}
// not nessacary just for fun
/*impl<'a> Default for Rectangle<'a> {
    fn default() -> Self {
        return Rectangle {
            width: 5.5,
            height: 7.7,
            next: None,
        };
    }
}*/
