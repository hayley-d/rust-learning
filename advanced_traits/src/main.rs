/*pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}*/

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(0);
    }
}

// generaics allow for mutiple implementations for the same type
/*impl Iterator<usize>  for Counter{
    fn next(&mut self) -> Option<usize> {
        return Some(0);
    }
}

impl Iterator<u32> for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        return Some(0);
    }
}*/

fn main() {
    let p1: Point = Point { x: 1, y: 1 };

    let p2: Point = Point { x: 2, y: 2 };

    assert_eq!(Point { x: 3, y: 3 }, p1 + p2);
}

// operator overloading
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

struct Millimeters {
    units: u32,
}

struct Meters {
    units: u32,
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        return Millimeters {
            units: self.units + other.units,
        };
    }
}

trait Jigglypuff {
    fn sing(&self);
}

trait MakkaPakka {
    fn sing(&self);
}

struct Sloth {
    name: String,
}

impl Sloth {
    fn sing(&self) {
        println!("singing");
    }
}

impl Jigglypuff for Sloth {
    fn sing(&self) {
        println!("singing");
    }
}

impl MakkaPakka for Sloth {
    fn sing(&self) {
        println!("singing");
    }
}

fn foo() {
    let sloth = Sloth {
        name: String::from("sloth"),
    };

    sloth.sing();
    Jigglypuff::sing(&sloth);
    MakkaPakka::sing(&sloth);
    <Sloth as Jigglypuff>::sing(&sloth);
}
