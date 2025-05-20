use std::error::Error;
use std::fs;

// Main application logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the contents of the file specified by the user
    let contents = fs::read_to_string(config.filename)?;

    // Output the file contents (will be replaced with actual search logic later)
    println!("With text:\n{}", contents);

    Ok(())
}

// Struct to hold configuration parsed from command-line arguments
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // Constructor function for Config
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Expect at least 3 arguments: program name, query string, and file name
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}
