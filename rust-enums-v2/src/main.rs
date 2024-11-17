enum Colours {
    Red,
    Green,
    Blue,
}

impl Colours {
    pub fn print(&self) {
        match self {
            Colours::Red => println!("Red"),
            Colours::Green => println!("Green"),
            _ => println!("Blue"),
        };
    }
}
fn main() {
    let colour: Colours = Colours::Red;
    colour.print();
}
