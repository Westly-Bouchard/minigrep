use std::{env, fs};

fn main() {
    // Collect the arguments passed to the executable from the command line as an array of strings
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let (query, filename) = parse_config(&args);

    // Test to make sure it works
    println!("Looking for {} in {}", query, filename);

    // Read the contents of the file
    let contents = fs::read_to_string(filename)
        .expect("Failed to read the contents of the file");
    
    // Test to make sure it works
    println!("With contents: {}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // Parse arguments, starts at 1 because the first argument is the name of the executable
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}