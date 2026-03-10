use std::error::Error;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // We store all arguments in data, a Vec<String>.
    let data: Vec<String> = env::args().collect();

    let search_str = &data[1];
    let file_name = &data[2];

    // We then read the file.
    let file_content = fs::read_to_string(file_name)?;

    // Print as a test.
    println!("The file has a length {:?}", file_content.len());
    Ok(())
}
