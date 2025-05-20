use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Collect the command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Parse the arguments into a Config struct, or handle errors gracefully
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    // Run the core logic, handling any potential runtime errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    };
}
