use std::{env, fs};

fn main() {
    // cargo run searchstring poem.txt
    let args: Vec<String> = env::args().collect();
    // capture our arguments in variables
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for '{}'", query);
    println!("In file {}", filename);
    let mut count = 0;
    for argument in &args {
        println!("{}: {}", count, argument);
        count += 1;
    }

    // read in the file
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
