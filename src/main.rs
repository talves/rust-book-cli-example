use std::env;

fn main() {
    // cargo run searchstring example-filename.txt
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut count = 0;
    for argument in args {
        println!("{}: {}", count, argument);
        count += 1;
    }
}
