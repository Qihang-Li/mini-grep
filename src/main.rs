use std::env;

fn main() {
    // We store all arguments in data, a Vec<String>
    let data: Vec<String> = env::args().collect();

    println!("Args: {:?}", &data[1..]);
}
