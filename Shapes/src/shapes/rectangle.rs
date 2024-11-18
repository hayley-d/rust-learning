use std::fmt::Display;

use crate::shapes::area::Area;
// this is the same
//use super::area::Area;

pub struct Rectangle<T> {
    pub width: T,
    pub height: T,
}

impl Area for Rectangle<f64> {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rectangle<f64> {
    fn default() -> Self {
        return Rectangle {
            width: 5.5,
            height: 7.7,
        };
    }
}

impl Display for Rectangle<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Rectangle({}x{})", self.width, self.height);
    }
}
