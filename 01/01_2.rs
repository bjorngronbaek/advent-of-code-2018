use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::collections::HashSet;
use std::vec::Vec;

fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut changes = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let value: i32 = line.parse().unwrap();
        changes.push(value);
    }

    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut sum = 0;
    let mut dup_freq = 0;
    let mut found_freq = false;

    while !found_freq {
        for change in &changes {
            sum = sum + change;

            if !found_freq && frequencies.contains(&sum) {
                dup_freq = sum;
                found_freq = true;
            }
            frequencies.insert(sum);
        }
    }

    if found_freq {
        println!("First duplicate freq: {}",dup_freq);
    }
    
    Ok(())
}

fn main () {
    let args: Vec<String> = env::args().collect();
    let _result = read_all_lines(&args[1]);    
}