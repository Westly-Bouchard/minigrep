use std::{env, fs};

fn main() {
    // Collect the arguments passed to the executable from the command line as an array of strings
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let config = Config::new(&args);

    // Test to make sure it works
    println!("Looking for {} in {}", config.query, config.filename);

    // Read the contents of the file
    let contents = fs::read_to_string(config.filename)
        .expect("Failed to read the contents of the file");
    
    // Test to make sure it works
    println!("With contents: {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config {query, filename}
    }
}