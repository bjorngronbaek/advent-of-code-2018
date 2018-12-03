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

    for line in reader.lines() {
        let line = line?;
        let (two_count,three_count) = has_pair(&line);
        if two_count == 1 {
            ctwo += 1;
        }
        if three_count == 1 {
            cthree += 1;
        }
    }

    let checksum = ctwo * cthree;
    println!("{}x{}={}",ctwo,cthree,checksum);

    Ok(())
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