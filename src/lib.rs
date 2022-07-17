use std::fs;
use std::error::Error;
use std::env;

pub fn run(config : Config) -> Result<(), Box<dyn Error>> { 
    //catch-all error handling for file reading
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };


    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub struct Config {
    pub query : String,
    pub filename : String,
    pub ignore_case : bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) 
    -> Result<Config, &'static str> {
        // takes ownership of an iterator instead of borrowing a &[String]
        // more efficient than having to clone borrowed strings

        args.next(); // first argument is program name, ignore
        
        let query = match args.next() {
            None => return Err("Didn't get a query string."),
            Some(arg) => arg
        };
        let filename = match args.next() {
            None => return Err("Didn't get a file name."),
            Some(arg) => arg
        };

        // checks if IGNORE_CASE environment variable is set; value is irrelevant
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filename, ignore_case })
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

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "ruST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}