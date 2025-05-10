use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}", err);
        process::exit(1)
    });

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

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        
        Ok(Config { query, filename })
    }
}
