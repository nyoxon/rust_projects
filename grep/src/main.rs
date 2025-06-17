extern crate grep;

use std::process;
use std::env; // enables us to read cl arguments

use grep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}