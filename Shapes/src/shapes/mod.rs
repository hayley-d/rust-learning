use crate::shapes::area::Area;

pub mod area;
pub mod circle;
pub mod rectangle;

impl Area for f64 {
    fn area(&self) -> f64 {
        return self * self;
    }
}
