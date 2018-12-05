extern crate aoc04;
use std::env;
use std::process;

use aoc04::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    aoc04::run(config);
}