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

// 3. The Reading Logic
pub fn read(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file.
    let contents = fs::read_to_string(&config.file_path)?;

    // Print as a test.
    println!("You are going to search text {:?}.", config.query);
    println!("The file has a length of {:?}.", contents.len());
    Ok(())
}

// 4. The Searching logic
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

// 5. Test Modules
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let test_query = "temparate";
        let test_contents = "\
        Shall I compare thee to a summer's day?
        Thou art more lovely and more temperate:
        Rough winds do shake the darling buds of May,
        And summer's lease hath all too short a date;";

        assert_eq!(
            search(test_query, test_contents),
            vec!["Thou art more lovely and more temperate:"]
        );
    }
}
