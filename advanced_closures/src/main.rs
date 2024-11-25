fn add_one(x: f64) -> f64 {
    return x + 1.0;
}

fn do_twice<T>(f: T, arg: f64) -> f64
where
    T: Fn(f64) -> f64,
{
    return f(arg) + f(arg);
}

// 3 closure traits:
// Fn
// FnMut
// FnOnce
fn main() {
    let answer: f64 = do_twice(add_one, 5.0);

    println!("The answer is {}", answer);
}

fn return_closure(a: f64) -> Box<dyn Fn(f64) -> f64> {
    if a > 0.0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}
