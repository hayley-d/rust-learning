use crate::Circle;
use crate::Rectangle;

/**
* Structures to define types of iterators.
*/
pub struct RectIter<'a> {
    pub current: Option<&'a Rectangle<'a>>,
    pub index: u32,
}

pub struct CircleIter<'a> {
    pub current: Option<&'a Circle<'a>>,
    pub index: u32,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct PointIter<'a> {
    pub points: &'a Vec<(f64, f64)>,
    pub index: u32,
}

/**
* Implementation for the Rectangle Iterator.
* Iterates through rectangles.
*/
impl<'a> RectIter<'a> {
    pub fn new(start: Option<&'a Rectangle<'a>>) -> Self {
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
    type Item = &'a Rectangle<'a>;

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

impl<'a> IntoIterator for Rectangle<'a> {
    type Item = &'a Rectangle<'a>;

    type IntoIter = RectIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return RectIter::new(self.next);
    }
}

impl<'a> CircleIter<'a> {
    pub fn new(start: Option<&'a Circle<'a>>) -> Self {
        return CircleIter {
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

impl<'a> Iterator for CircleIter<'a> {
    type Item = &'a Circle<'a>;

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

impl<'a> IntoIterator for Circle<'a> {
    type Item = &'a Circle<'a>;

    type IntoIter = CircleIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return CircleIter::new(self.next);
    }
}

impl<'a> PointIter<'a> {
    pub fn new(start: &'a Vec<Point>) -> Self {
        return PointIter {
            points: start,
            index: 0,
        };
    }

    /*pub fn get_point(&self, idx: u32) -> Option<&Point> {
        return match self.points.get(idx) {
            Some(x) => *x,
            None => None,
        };
    }*/
}
/*
impl<'a> Iterator for PointIter<'a> {
    type Item = &'a Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.points[self.index];
        self.index += 1;
        return curr;
    }
}*/

impl<'a> From<Vec<(f64, f64)>> for PointIter<'a> {
    fn from(points: Vec<(f64, f64)>) -> Self {
        return PointIter { points, index: 0 };
    }
}
