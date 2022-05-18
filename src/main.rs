use std::env;

fn main() {
    // Collect the arguments passed to the executable from the command line as an array of strings
    let args: Vec<String> = env::args().collect();
    
    // Parse the query and the name of the file we want to search from the arguments
    // Starting at 1 b/c the first element will be the name of the executable
    let query = &args[1];
    let filename = &args[2];

    // Test to make sure it works
    println!("Looking for {} in {}", query, filename);
}
