enum IpAddressType {
    V4(u8, u8, u8, u8),
    V6(u8, u8, u8, u8),
}

/**
* This is better than a struct in this senrio since you can declare associated subtypes without
* multiple structs.
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn get_message(&self) -> String {
        return self.Write;
    }
}

struct IpAddress {
    address_type: IpAddressType,
    address: String,
}

fn main() {
    //let ip_v4: IpAddressType = IpAddressType::V4(String::new());
    let ip_v6: IpAddressType = IpAddressType::V6(String::new());

    let add1: IpAddress = IpAddress {
        address_type: IpAddressType::V4(String::from("127.0.0.1")),
        address: String::from("127.0.0.1"),
    };

    let localhost: IpAddressType = IpAddressType::V4(127, 0, 0, 1);
}

fn route(ip_type: IpAddressType) {
    //do something
}

// This is what the option<T> enum looks like since Rust does not have a null type
/*enum Option<T> {
    Some(T),
    None,
}*/

fn main2() {
    let num: i32 = 7;
    let some_number: Option<i32> = Some(7);

    let some_string: Option<String> = None;

    let sum: i32 = match some_number {
        Some(x) => num + x,
        None => num,
    };
}

enum Food {
    Burger,
    Pizza,
    Waffle,
}

fn match_the_food(food: Food) -> String {
    return match food {
        Food::Burger => String::from("Burger"),
        Food::Pizza => String::from("Pizza"),
        Food::Waffle => String::from("Waffle"),
    };
}
