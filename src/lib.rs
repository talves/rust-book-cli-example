use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read in the file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
    println!("Searching for: {}", config.query);

    Ok(()) // if we get this far everything is ok
}
