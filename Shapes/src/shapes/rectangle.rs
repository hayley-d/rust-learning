use std::fmt::Display;

use crate::shapes::area::Area;
// this is the same
//use super::area::Area;

pub struct Rectangle<'a, T> {
    pub width: T,
    pub height: T,
    pub next: Option<&'a Rectangle<'a, T>>,
}

impl<'a> Rectangle<'a, f64> {
    pub fn set_next(&mut self, other: Option<&'a Rectangle<'a, f64>>) {
        self.next = other;
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        return (self.width, self.height) == (x, y);
    }
}
impl<'a> Area for Rectangle<'a, f64> {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl<'a> Default for Rectangle<'a, f64> {
    fn default() -> Self {
        return Rectangle {
            width: 5.5,
            height: 7.7,
            next: None,
        };
    }
}

impl<'a> Display for Rectangle<'a, f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Rectangle({}x{})", self.width, self.height);
    }
}

pub struct RectIter<'a> {
    pub current: Option<&'a Rectangle<'a, f64>>,
    pub index: u32,
}

impl<'a> RectIter<'a> {
    pub fn new(start: Option<&'a Rectangle<'a, f64>>) -> Self {
        return RectIter {
            current: start,
            index: 0,
        };
    }

    pub fn has_next(&self) -> bool {
        return match self.current {
            Some(x) => {
                return match x.next {
                    Some(_) => true,
                    None => false,
                }
            }
            None => false,
        };
    }
}

impl<'a> Iterator for RectIter<'a> {
    type Item = &'a Rectangle<'a, f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let curr = self.current;
        self.current = match self.current {
            Some(x) => x.next,
            None => None,
        };
        return curr;
    }
}

impl<'a> IntoIterator for Rectangle<'a, f64> {
    type Item = &'a Rectangle<'a, f64>;

    type IntoIter = RectIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return RectIter::new(self.next);
    }
}
