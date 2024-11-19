use crate::shapes::area::Area;
use std::f64::consts::PI;
use std::fmt::Display;

pub struct Circle<T> {
    pub radius: T,
    pub x: T,
    pub y: T,
}

impl Circle<f64> {
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        return (self.x, self.y) == (x, y);
    }
}

impl Area for Circle<f64> {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Default for Circle<f64> {
    fn default() -> Self {
        return Circle {
            radius: 5.5,
            x: 7.7,
            y: 7.7,
        };
    }
}

impl Display for Circle<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle({})", self.radius);
    }
}
