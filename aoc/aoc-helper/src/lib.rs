use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;
use std::error::Error;

#[cfg(test)]
mod tests {
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let reader = io::BufReader::new(file);
    
    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        entries.push(line);
    }

    Ok(entries)
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}