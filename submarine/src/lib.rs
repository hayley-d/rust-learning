pub mod submarine {
    pub struct Submarine {
        pub x: isize,
        pub y: isize,
    }

    impl Submarine {
        pub fn new() -> Submarine {
            return Submarine { x: 0, y: 0 };
        }

        pub fn forward(&mut self, x: isize) {
            self.x += x;
        }

        pub fn backward(&mut self, x: isize) {
            self.x -= x;
        }

        pub fn up(&mut self, y: isize) {
            self.y -= y;
        }

        pub fn down(&mut self, y: isize) {
            self.y += y;
        }

        pub fn get_coordinates(&self) -> (isize, isize) {
            return (self.x, self.y);
        }
    }
}
