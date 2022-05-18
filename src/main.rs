use std::{env, fs, process, error::Error};

fn main() {
    // Collect the arguments passed to the executable from the command line as an array of strings
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Test to make sure it works
    println!("Looking for {} in {}", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read contents to string
    let contents = fs::read_to_string(config.filename)?;

    // Test to make sure it works
    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // We need three total args to successfully run the program
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Grab the query and filename from the arguments 
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}