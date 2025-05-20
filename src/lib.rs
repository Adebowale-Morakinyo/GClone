use std::error::Error;
use std::fs;

// Main application logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the contents of the file specified by the user
    let contents = fs::read_to_string(config.filename)?;

    // Output lines found
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
