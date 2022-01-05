use std::error::Error;
use std::process;
use std::{env, fs};

fn main() {
    // cargo run searchstring poem.txt
    let args: Vec<String> = env::args().collect();

    // made and used a constructor instead
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Fatal Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Must have 2 arguments. Example: cargo run <query> <filename>");
        }
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

        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read in the file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    println!("Searching for: {}", config.query);

    Ok(()) // if we get this far everything is ok
}
