use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    let config: Config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents)
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let filename: String = args[2].clone();
    
    Config { query, filename }
}
