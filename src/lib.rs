use std::fs;
use std::error::Error;

pub fn run(config : Config) -> Result<(), Box<dyn Error>> { 
    //catch-all error handling for file reading
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results : Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}