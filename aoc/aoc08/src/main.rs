extern crate aoc_helper;
extern crate aoc08;

use std::env;
use std::process;

use aoc_helper::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let mut entries = aoc_helper::run(config).unwrap();
    let line = entries.pop().unwrap();    
    let ciphers = line.split(" ");
    let vec = ciphers.collect::<Vec<&str>>();

    let s = aoc08::split_into_nodes(vec);
    println!("{}",s);
}

fn get_serial_sum(serial: &str) -> usize {
    0
}