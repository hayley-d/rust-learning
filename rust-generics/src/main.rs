fn main() {
    let number_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    println!("The vector is {:#?}", number_list);
    println!("The largest number in the vector is {}", largest);
}

fn get_max_element<T: Ord + Copy>(list_of_items: &Vec<T>) -> Option<T> {
    return list_of_items.iter().max();
}
