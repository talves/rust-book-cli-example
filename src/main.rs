use std::env;

fn main() {
    // cargo run searchstring example-filename.txt
    let args: Vec<String> = env::args().collect();
    // capture our arguments in variables
    let query = &args[1];
    let filename = &args[2];
    println!("{:?}", args);
    let mut count = 0;
    for argument in args {
        println!("{}: {}", count, argument);
        count += 1;
    }
}
