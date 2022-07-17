use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // read input arguments from stdin
    let args: Vec<String> = env::args().collect();
    // use a closure (anonymous function) to exit the process on error
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }  
}