use std::env;
use std::process;
use std::thread;
use std::time::Duration;

use rust_book_cli_example::{run, Config};

fn main() {
    // cargo run searchstring poem.txt
    let args: Vec<String> = env::args().collect();

    simulated_expensive_calculation(20);

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

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
