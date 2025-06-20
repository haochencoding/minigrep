use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Search {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).unwrap();
    println!("{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic!("No enough arguments. Please specify search query and file path.")
        }
        let query = args.get(1).unwrap().clone();
        let file_path = args.get(2).unwrap().clone();

        Self { query, file_path }
    }
}
