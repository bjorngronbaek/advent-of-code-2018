use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::collections::HashSet;

fn read_all_lines(filename: &str) -> io::Result<()> {    
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;

        let value: i32 = line.parse().unwrap();
        sum = sum + value;

        println!("{}\t{}", line,sum);
    }

    println!("Sum: {}",sum);
    Ok(())
}

fn main () {
    let args: Vec<String> = env::args().collect();
    let _result = read_all_lines(&args[1]);    
}