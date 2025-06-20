use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str > {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args.get(1).unwrap().clone();
        let file_path = args.get(2).unwrap().clone();

        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("{contents}");
    Ok(())
}