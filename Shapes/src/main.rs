use std::f64::consts::PI;
mod shapes;
use shapes::{Circle, Rectangle};

trait Area {
    fn area(&self) -> f64 {
        return 0.0;
    }
}

impl Area for Rectangle<f64> {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Area for Circle<f64> {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

fn main() {
    let rec: Rectangle<f64> = Rectangle {
        width: 6.5,
        height: 5.5,
    };

    println!("The area of the rectangle is {}cm^2", get_area(rec));
}

fn get_area(item: impl Area) -> f64 {
    return item.area();
}
