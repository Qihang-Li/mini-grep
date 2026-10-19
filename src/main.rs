use mini_grep::Config;
use std::{env, process};

fn main() {
    // Store all arguments in data, a Vec<String>.
    let data: Vec<String> = env::args().collect();

    // Main task 1: Build the search configuration.
    // Error handling 1: Handle invalid arguments.
    // Get the Config value, or print the error and exit.
    let config = Config::build(&data).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    // Main task 2: Read the file, search it, and print matching lines.
    // Error handling 2: Handle errors returned by run.
    // We only need to handle the error here.
    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
