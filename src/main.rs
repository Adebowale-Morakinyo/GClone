use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Parse the arguments into a Config struct, or handle errors gracefully
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    // Run the core logic, handling any potential runtime errors
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    };
}

// Main application logic
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the contents of the file specified by the user
    let contents = fs::read_to_string(config.filename)?;

    // Output the file contents (will be replaced with actual search logic later)
    println!("With text:\n{}", contents);

    Ok(())
}

// Struct to hold configuration parsed from command-line arguments
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // Constructor function for Config
    fn new(args: &[String]) -> Result<Config, &str> {
        // Expect at least 3 arguments: program name, query string, and file name
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}
