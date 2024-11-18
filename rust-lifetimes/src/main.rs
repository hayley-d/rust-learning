use std::fmt::Display;

fn main() {
    let r: &i32;

    {
        let x: i32 = 7;
        r = &x;
        println!("{}", *r);
    }

    let s1: String = String::from("abcd");
    {
        let s2: String = String::from("xyz");
        let result: &str = longest(&s1, &s2);
        println!("{}", s1);
        println!("The longest string is {}", result);
    }

    // by  all string slices have a static lifetime that lasts till the end of the program
    let static_string: &'static str = "I love sloths";
}

// all have the same lifetimes
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    return if x.len() > y.len() { x } else { y };
}

// only uses the lifetime of x
pub fn smallest<'a>(x: &'a str, y: &str) -> &'a str {
    return x;
}

struct ImportantStruct<'a> {
    part: &'a str,
}

// Third Rule: If one of the params is a ref to self or mutable ref to self then all lifetimes are
// the same as self
impl<'a> ImportantStruct<'a> {
    fn return_part(&self, x: &str) -> &str {
        return self.part;
    }

    fn return_parts(&self, x: &'a str) -> &'a str {
        return self.part;
    }
}

struct Book {
    part: String,
}

fn format_my_struct() {
    let novel: String = String::from("The Little Prince");
    let first_sen: &str = match novel.split(' ').next() {
        Some(s) => s,
        None => "",
    };

    let i: ImportantStruct = ImportantStruct { part: first_sen };
}

// Each param gets its own lifetime
// If there is one input then the lifetime is the same for the output
// If there are multiple lifetime params

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
