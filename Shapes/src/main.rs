mod shapes;
use shapes::{area::Area, circle::Circle, rectangle::Rectangle};

fn main() {
    let mut rec1: Rectangle<f64> = Rectangle {
        width: 6.5,
        height: 5.5,
        next: None,
    };

    let rec2: Rectangle<f64> = Rectangle {
        width: 6.5,
        height: 5.5,
        next: Some(&rec1),
    };

    let rec: Rectangle<f64> = Rectangle {
        width: 6.5,
        height: 5.5,
        next: Some(&rec2),
    };

    let cir: Circle<f64> = Circle {
        radius: 4.4,
        x: 3.3,
        y: 3.3,
    };

    //println!("The area of the rectangle is {}cm^2", get_area(rec));
    println!("{}", rec);
    println!("The area of the circle is {}cm^2", get_area(cir));
    println!("The area of the float is {}cm^2", get_area(6.9));

    for r in rec {
        println!("{}", r);
    }
}

fn get_area(item: impl Area) -> f64 {
    return item.area();
}
