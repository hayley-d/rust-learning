use std::fmt::Display;

pub trait Summary {
    // default impl
    fn summarize(&self) -> String {
        return String::new();
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // uses defult impl
    /*fn summarize(&self) -> String {
        return format!("{}, by {}", self.headline, self.author);
    }*/
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub length: u32,
}

impl Summary for Tweet {
    // overrides the default impl
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

// requires the type implements the trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// basically syntax sugar for
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// different trait contditions
pub fn foo(item1: impl Summary + Display, item2: impl Ord + Copy) {}

fn main() {
    let tweet: Tweet = Tweet {
        username: "hayley-d".to_string(),
        content: String::from("Sloths are the best"),
        length: 32,
    };

    let article: NewsArticle = NewsArticle {
        author: String::from("Hayley D"),
        headline: String::from("Sloths are Amazing!"),
        content: String::new(),
    };

    println!("{}", tweet.summarize());
    println!("{}", article.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        return Self { x, y };
    }
}

impl<T: Display + Ord> Pair<T> {
    fn cmp_vars(&self) {
        if self.x >= self.y {
            println!("The larger variable is {}", self.x);
        } else {
            println!("The larger variable is {}", self.y);
        }
    }
}
