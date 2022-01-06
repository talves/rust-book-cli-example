use std::env;
use std::process;

use rust_book_cli_example::generate_workout;
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
