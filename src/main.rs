use minigrep::Config;
use std::{env, process};

// 4. The Organizer
fn main() {
    // Store all arguments in data, a Vec<String>.
    let data: Vec<String> = env::args().collect();

    // Main task 1: Fetch the input from arguments.
    // Error handling 1: Data Extraction
    // We want to extract a Config result here.
    let config = Config::build(&data).unwrap_or_else(|e| {
        println!("{e}");
        process::exit(1);
    });

    // Main task 2: Read the file from the given filename.
    // Error handling 2: Side-Effect Verification
    // We want to extract the error only.
    if let Err(e) = minigrep::read(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // It's equivalent to this code below:
    // read(config).unwrap_or_else(|err| {
    //    eprintln!("Application error: {err}");
    //    process::exit(1);
    //});
    // But it's not considered idiomatic in Rust,
}
