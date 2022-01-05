use std::{env, fs};

fn main() {
    // cargo run searchstring poem.txt
    let args: Vec<String> = env::args().collect();

    let config = parse_arguments(&args);

    // read in the file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    println!("Searching for: {}", config.query);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_arguments(args: &[String]) -> Config {
    // capture our arguments in variables
    let query = args[1].clone();
    let filename = args[2].clone();
    println!("Searching for '{}'", query);
    println!("In file {}", filename);

    // output list with index
    let mut count = 0;
    for argument in args {
        println!("{}: {}", count, argument);
        count += 1;
    }

    Config { query, filename }
}
