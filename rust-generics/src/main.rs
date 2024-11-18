use std::collections::HashMap;

fn make_a_hashmap<T: Ord + Copy, H: Ord + Copy>() -> HashMap<T, H> {
    let hashmap: HashMap<T, H> = HashMap::new();
    return hashmap;
}

struct Building<T> {
    people: T,
    capacity: u32,
}

fn main() {
    let number_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    let max: i32 = match get_max_element(&number_list) {
        Some(n) => n,
        None => 0,
    };

    println!("The vector is {:#?}", number_list);
    println!("The largest number in the vector is {}", max);

    let building: Building<String> = Building {
        people: String::new(),
        capacity: 7,
    };
}

fn get_max_element<T: Ord + Copy>(list_of_items: &Vec<T>) -> Option<T> {
    return list_of_items.iter().max().copied();
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn print_points(&self) -> &T {
        return &self.x;
    }
}

impl Point<f32> {
    pub fn print_num(&self) {
        println!("{};{}", self.x, self.y);
    }
}

impl Point<i32> {
    pub fn print_num(&self) {
        println!("{};{}", self.x, self.y);
    }
}
