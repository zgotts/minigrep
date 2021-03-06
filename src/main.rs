extern crate minigrep; // pulls lib crate into scope

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: env::Args = env::args();
    
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}