use std::env;
use std::process;
use std::thread;
use std::time::Duration;

use rust_book_cli_example::{run, Config};

fn main() {
    // cargo run searchstring poem.txt
    let args: Vec<String> = env::args().collect();

    generate_workout(12, 3);
    generate_workout(30, 12);

    // made and used a constructor instead
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Fatal Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// definition of the Cacher struct that holds a closure and an optional result value
// see ch. 13.1 of rust book https://doc.rust-lang.org/book/ch13-01-closures.html
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
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}
