use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Search {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path).unwrap();
    println!("{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str > {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args.get(1).unwrap().clone();
        let file_path = args.get(2).unwrap().clone();

        Ok(Self { query, file_path })
    }
}
