pub mod drawer {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        // dyn = dynamic dispatch
        // a vector or trait objects
        // trait objs need a pointer
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for comp in self.components.iter() {
                comp.draw();
            }
        }

        pub fn add_component(&mut self, component: Box<dyn Draw>) {
            self.components.push(component);
        }
    }

    pub struct Button {
        pub width: usize,
        pub height: usize,
        pub label: String,
    }

    impl Button {
        pub fn new() -> Button {
            return Button {
                width: 70,
                height: 40,
                label: String::from("Button"),
            };
        }
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("The button is {} x {}", self.width, self.height);
        }
    }

    pub struct TextBox {
        pub width: usize,
        pub height: usize,
        pub placeholder: String,
    }

    impl TextBox {
        pub fn new() -> TextBox {
            return TextBox {
                width: 70,
                height: 40,
                placeholder: String::from("Text here"),
            };
        }
    }

    impl Draw for TextBox {
        fn draw(&self) {
            println!("The textbox is {} x {}", self.width, self.height);
        }
    }
}

// You would think that you could instead define a generic of T:Draw which would not be wrong but
// in the components vector it could only store a single type so here it could only be a vector or
// buttons or textboxes where as with trait objects it can be a combination of different types that
// implement the Draw trait
