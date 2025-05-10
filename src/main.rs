use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Searching for {}", query);
    println!("In File {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents)
}
