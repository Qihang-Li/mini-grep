use std::error::Error;
use std::{env, fs};

struct Config {
    // query is the text to be searched,
    // and file_path is the path to a certain file.
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    Config {
        query: args[1].to_string(),
        file_path: args[2].to_string(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // We store all arguments in data, a Vec<String>.
    let data: Vec<String> = env::args().collect();

    // We parse the arguments into a Config struct.
    let config = parse_config(&data);

    // We then read the file.
    let file_content = fs::read_to_string(config.file_path)?;

    // Print as a test.
    println!("You are going to search text {:?}", config.query);
    println!("The file has a length {:?}", file_content.len());
    Ok(())
}
