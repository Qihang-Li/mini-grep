use std::error::Error;
use std::fs;

// 1. Data Structure
pub struct Config {
    // query is the text to be searched,
    // and file_path is the path to a certain file.
    query: String,
    file_path: String,
}

// 2. Set up logic
impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("Wrong input. Please specify the string and file to search.");
        } else {
            Ok(Self {
                query: args[1].to_string(),
                file_path: args[2].to_string(),
            })
        }
    }
}

// 3. The Execution Logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file.
    let contents = fs::read_to_string(&config.file_path)?;

    // Print as a test.
    println!("You are going to search text {:?}.", config.query);
    println!("The file has a length of {:?}.", contents.len());
    Ok(())
}
