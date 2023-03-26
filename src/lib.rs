use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir().expect("Fatal");
    println!("Current dir is {}", current_dir.display());

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        let args_len = args.len() - 1;
        if args_len < 2 {
            let msg = format!("{args_len} arguments instead of 2");
            return Err(msg);
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
