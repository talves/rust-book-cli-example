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

    println!("search vec:\n{:?}", search(&config.query, &contents));
    println!("Searching for: {}", config.query);
    for line in search(&config.query, &contents) {
        println!(" {}", line);
    }

    Ok(()) // if we get this far everything is ok
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// Tests!!!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
