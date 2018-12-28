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
    let vec = ciphers.map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    println!("{:?}",vec);

    let nodevalue = aoc08::sub(&vec,0);

    println!("Final serial sum is: {}",nodevalue.newsum);
    println!("Final value is: {}",nodevalue.newvalue);
}