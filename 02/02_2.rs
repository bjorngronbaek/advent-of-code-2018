use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::vec::Vec;
use std::collections::HashMap;

fn main(){
    let args: Vec<String> = env::args().collect();
    let _lines = read_all_lines(&args[1]);
}

fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut ctwo = 0;
    let mut cthree = 0;

    let mut ids = Vec::new();
    for line in reader.lines() {
        let line = line?;        
        ids.push(line);
    }

    for i in 0..ids.len() {
        for j in (i+1)..ids.len() {
            println!("Compairing {} and {}:\t {}",ids[i],ids[j],distance(&ids[i],&ids[j]));
        }        
    }

    Ok(())
}

fn distance(first: &str, second: &str) -> i32 {
    let mut distance = 0;
    for (f,s) in first.chars().zip(second.chars()) {
        if f != s {
            distance += 1;
        }
    }

    distance
}

fn has_pair(s: &str) -> (i32,i32) {
    let mut count_map = HashMap::new(); // : HashMap<&char,i32>
    for c in s.chars() {
        let count = count_map.entry(c).or_insert(0);
        *count += 1;
    }
    //println!("{:?}",count_map);

    let mut two_count = 0;
    let mut three_count = 0;
    for (_key,value) in count_map {
        if value == 2 {
            two_count = 1;
        }

        if value == 3 {
            three_count = 1;
        }
    }

    println!("In string {}:\t Found twos {} and threes {}",s,two_count,three_count);

    (two_count,three_count)    
}