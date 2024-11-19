use crate::shapes::area::Area;
use std::f64::consts::PI;
use std::fmt::Display;

pub struct Circle<'a> {
    pub radius: f64,
    pub x: f64,
    pub y: f64,
    pub next: Option<&'a Circle<'a>>,
}

impl Circle<'_> {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let x = self.x - x;
        let y = self.y - y;
        return (x * x + y * y) <= self.radius * self.radius;
    }
}

impl Area for Circle<'_> {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

/*impl Default for Circle {
    fn default() -> Self {
        return Circle {
            radius: 5.5,
            x: 7.7,
            y: 7.7,
        };
    }
}*/

impl Display for Circle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle({})", self.radius);
    }
}
