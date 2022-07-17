use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // read input arguments from stdin
    // use a closure (anonymous function) to exit the process on error
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }  
}