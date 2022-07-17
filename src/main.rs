use std::env;
use std::fs;
use std::process;

fn main() {
    // read input arguments from stdin
    let args: Vec<String> = env::args().collect();
    // use a closure (anonymous function) to exit the process on error
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file.");

    println!("With text:\n{contents}");
}

struct Config {
    query : String,
    filename : String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Must have a query and filename.");
        }

        let query = args[1].to_string();
        let filename = args[2].to_string();

        Ok(Config { query, filename })
    }
}