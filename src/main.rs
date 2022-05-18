use std::{env, process};
use minigrep::Config;

fn main() {    
    // Parse command line arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Test to make sure it works
    println!("Looking for {} in {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}