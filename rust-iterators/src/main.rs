pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    //other methods
}


#[test]
fn iterator_test() {
    let v1 : Vec<i32> = vec![1,2,3,4,5];

    let mut it : Iter<i32> = v1.iter();

    assert_eq!(it.next(),Some(1);
    todo!("Finish later");
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4];

    for str_num in numbers.iter().map(|x| x + 1) {
        println!("{}", str_num);
    }

    let mut foo = numbers.iter().map(|x| x + 1);
    let mut new_vector = vec![];

    while let Some(x) = foo.next() {
        new_vector.push(x);
    }

    println!("{:?}", &foo);

    //gathers the string into a string
    let collection_of_strings: String = vec!["Hello", "World", "Love", "Sloths"]
        .into_iter()
        .collect();

    // convert a vector into a hashmap
    let foo: HashMap<&str, i32> = vec!["Hello", "World", "Love", "Sloths"]
        .into_iter()
        .enumerate()
        .map(|(index, item)| (item, index))
        .collect();
}
