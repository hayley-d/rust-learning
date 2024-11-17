struct User {
    username: String,
    email: String,
    age: u32,
    online: bool,
}

fn main() {
    let mut user1: User = User {
        username: "Hayley",
        email: "hayleydod@proton.me",
        age: 22,
        online: true,
    };

    let username: String = user1.username;
    user1.username = "haylo";

    let user2: User = user_factory(String::from("Sloth"), String::from("email@gamil.com"), 8);

    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn user_factory(username: String, email: String, age: u32) -> User {
    return User {
        username,
        email,
        age,
        online: false,
    };
}

struct Rectangle {
    width: f32,
    height: f32,
}

fn area_of_rectangle(rec: Rectangle) -> f32 {
    return rec.width * rec.height;
}
