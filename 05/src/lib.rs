use std::fs;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;
use std::error::Error;

extern crate regex;
use regex::Regex;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("In file {}", config.filename);

    let mut contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let mut v : Vec<char> = contents.chars().collect();    

    for (i,c) in v.iter().enumerate() {
        println!("{}:{}",i,c);
        if i > 0 {
            let f = v.get(i-1).unwrap();
            if f.is_lowercase() && c.is_uppercase() || f.is_uppercase() && c.is_lowercase() {
                println!("found potential match: {},{}",f,c);
                if f.to_lowercase().to_string() == c.to_lowercase().to_string() {                    
                    /*
                    let mut rep = String::new();
                    rep.push(*f);
                    rep.push(*c);
                    println!("match! {}",rep);
                    println!("new string: {}",contents.replace(rep,""))
                    */
                    v.remove(i);
                    v.remove(i-1);
                    break;
                }
            }
        }
    }

    for (i,c) in v.iter().enumerate() {
        println!("{}:{}",i,c);
    }

    Ok(())
}