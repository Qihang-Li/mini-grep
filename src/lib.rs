use std::env;
use std::error::Error;
use std::fs;

// 1. Data structure
/// The query, file path, and search mode for a search.
pub struct Config {
    // query is the text to search for,
    // and file_path is the path to the file to search.
    query: String,
    file_path: String,
    ignore_case: bool,
}

// 2. Configuration setup
impl Config {
    /// Builds the search configuration from command-line arguments.
    ///
    /// `args` must contain the program name, a query, and a file path,
    /// in that order.
    ///
    /// # Errors
    ///
    /// Returns an error unless `args` contains exactly three elements.
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            Err("Expected exactly two arguments: <QUERY> <FILE_PATH>.")
        } else {
            Ok(Self {
                query: args[1].to_string(),
                file_path: args[2].to_string(),
                ignore_case: env::var("IGNORE_CASE").is_ok(),
            })
        }
    }
}

// 3. The main logic
/// Reads the file, searches its contents, and prints matching lines to stdout.
///
/// # Errors
///
/// Returns an error if the file cannot be read or contains invalid UTF-8.
///
/// # Panics
///
/// Panics if writing to stdout fails.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file.
    let contents = fs::read_to_string(&config.file_path)?;

    // Find and print matching lines.
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for item in result {
        println!("{item}");
    }
    Ok(())
}

// 4. Search logic
/// Finds lines containing `query`, matching case exactly.
///
/// Returns matching lines borrowed from `contents` in their original order.
/// Returns an empty vector if no lines match.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect()
}

/// Finds lines containing `query`, ignoring case.
///
/// Compares lowercase versions of the query and each line.
/// Returns matching lines borrowed from `contents`, keeping their original
/// text and order.
/// Returns an empty vector if no lines match.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();
    contents
        .lines()
        .filter(|&line| line.to_lowercase().contains(&lower_query))
        .collect()
}

// A. Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_rejects_extra_arguments() {
        let args = vec![
            String::from("mini_grep"),
            String::from("temper"),
            String::from("Sonnet_18.txt"),
            String::from("Zen_of_Python"),
        ];

        assert!(Config::build(&args).is_err());
    }

    #[test]
    fn case_sensitive() {
        let test_query = "temper";
        let test_contents = "\
temper first
TEMPER second
unrelated line
temper third";

        assert_eq!(
            search(test_query, test_contents),
            vec!["temper first", "temper third"]
        );
    }

    #[test]
    fn case_insensitive() {
        let test_query = "tEmpEr";
        let test_contents = "\
temper first
TEMPER second
unrelated line
Temper third
tEmPeR fourth";

        assert_eq!(
            search_case_insensitive(test_query, test_contents),
            vec![
                "temper first",
                "TEMPER second",
                "Temper third",
                "tEmPeR fourth",
            ]
        );
    }

    #[test]
    fn no_matches() {
        let test_query = "temper";
        let test_contents = "\
alpha
BETA
gamma";

        assert!(search(test_query, test_contents).is_empty());
        assert!(search_case_insensitive(test_query, test_contents).is_empty());
    }
}
