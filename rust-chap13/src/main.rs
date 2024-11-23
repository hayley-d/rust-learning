use std::thread;
use std::time::Duration;

fn simulated_expensive_calc(intensity: u32) -> u32 {
    println!("Claculating slowly.....");
    thread::sleep(Duration::from_secs(2));
    return intensity;
}

fn main() {
    let simulated_intensity: u32 = 10;
    let simulated_random_num: u32 = 7;

    generate_workout(simulated_intensity, simulated_random_num);
}

fn original_generate_workout(intensity: u32, random_num: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} hack squats!",
            simulated_expensive_calc(intensity)
        );
        println!(
            "Next, do {} back squats!",
            simulated_expensive_calc(intensity)
        );
    } else {
        if random_num == 3 {
            println!("Take a break today!, Remember to stay hydrated");
        } else {
            println!("Today, run for {} km", simulated_expensive_calc(intensity));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        return Cacher {
            calculation,
            value: None,
        };
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v: u32 = (self.calculation)(arg);
                self.value = Some(v);
                return v;
            }
        }
    }
}

fn generate_workout(intensity: u32, random_num: u32) {
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("Claculating slowly.....");
        thread::sleep(Duration::from_secs(2));
        return num;
    });

    if intensity < 25 {
        println!("Today, do {} hack squats!", cached_result.value(intensity));
        println!("Next, do {} back squats!", cached_result.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today!, Remember to stay hydrated");
        } else {
            println!("Today, run for {} km", cached_result.value(intensity));
        }
    }
}

/*pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}*/

#[test]
fn iterator_test() {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&5));
    assert_eq!(v1_iter.next(), None);
}

#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    return shoes.into_iter().filter(|s| s.size == shoe_size).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoe_size_test() {
        let shoes: Vec<Shoe> = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 4,
                style: String::from("boot"),
            },
            Shoe {
                size: 6,
                style: String::from("crocs"),
            },
            Shoe {
                size: 6,
                style: String::from("heel"),
            },
            Shoe {
                size: 8,
                style: String::from("slipper"),
            },
        ];

        assert_eq!(
            shoes_in_my_size(shoes, 6),
            vec![
                Shoe {
                    size: 6,
                    style: String::from("crocs"),
                },
                Shoe {
                    size: 6,
                    style: String::from("heel"),
                }
            ]
        );
    }
}
