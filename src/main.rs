use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Search {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).unwrap();
    println!("{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args.get(1).unwrap().clone();
    let file_path = args.get(2).unwrap().clone();

    Config { query, file_path }
}
