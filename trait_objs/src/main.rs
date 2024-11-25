use trait_objs::drawer::*;

struct SelectBox {
    pub width: usize,
    pub height: usize,
    pub selected: bool,
}

impl SelectBox {
    pub fn new() -> SelectBox {
        return SelectBox {
            width: 70,
            height: 40,
            selected: false,
        };
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("The select box is {} x {}", self.width, self.height);
    }
}
fn main() {
    let mut screen: Screen = Screen {
        components: Vec::new(),
    };

    screen.add_component(Box::new(Button::new()));
    screen.add_component(Box::new(TextBox::new()));
    screen.add_component(Box::new(SelectBox::new()));

    screen.run();
}
