extern crate aoc05;
use std::env;
use std::process;

use aoc05::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    aoc05::run(config);
}