use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::collections::HashSet;

fn read_all_lines(filename: &str) -> io::Result<()> {    
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let mut dup_freq = 0;
    let mut found_freq = false;
    for line in reader.lines() {
        let line = line?;

        let value: i32 = line.parse().unwrap();
        sum = sum + value;

        println!("{}\t{}", line,sum);

        if !found_freq && frequencies.contains(&sum) {
            println!("First duplicate freq: {}",sum);            
            dup_freq = sum;
            found_freq = true;
        }
        frequencies.insert(sum);
    }

    println!("Sum: {}",sum);
    if found_freq {
        println!("First duplicate freq: {}",dup_freq);
    }
    else {
        read_all_lines(&filename,&frequencies);
    }
    Ok(())
}

fn main () {
    let args: Vec<String> = env::args().collect();
    let mut frequencies = HashSet<>::new();
    let _result = read_all_lines(&args[1],frequencies);    
}