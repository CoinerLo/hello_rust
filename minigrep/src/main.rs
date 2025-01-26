use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let context = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{context}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}

