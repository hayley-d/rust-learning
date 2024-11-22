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
