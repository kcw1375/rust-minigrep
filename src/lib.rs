use std::fs;
use std::error::Error;

pub fn run(config : Config) -> Result<(), Box<dyn Error>> { 
    //catch-all error handling for file reading
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{contents}");
    Ok(())
}

pub struct Config {
    pub query : String,
    pub filename : String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Must have a query and filename.");
        }

        let query = args[1].to_string();
        let filename = args[2].to_string();

        Ok(Config { query, filename })
    }
}