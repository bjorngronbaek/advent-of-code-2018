extern crate aoc03;
extern crate regex;

use aoc03::Claim;
use aoc03::Piece;
use regex::Regex;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let claims : Vec<Claim> = read_all_lines(&args[1]).unwrap();

    find_counts(&claims);
}

fn find_counts(claims: &Vec<Claim>) {
    let mut piece = Piece::new(1000,1000);
    for claim in claims {
        piece.claim(claim);
    }

    //println!("{}",piece.render());

    let mut count = 0;
    for claim_count in &piece.claim_count {
        if *claim_count >= 2 {
            count += 1;
        }
    }

    println!("{} inches claimed by two or more elves",count);

    for claim in claims {
        if piece.unique_claim(claim){
            println!("id: {}",claim.id);
        }
    }
}

fn read_all_lines(filename: &str) -> io::Result<Vec<Claim>> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    //#1 @ 257,829: 10x23
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut claims: Vec<Claim> = vec![];

    for line in reader.lines() {
        let line = line?;
        //println!("{}",&line);
        for capture in re.captures_iter(&line) {
            let id :u32 = capture[1].parse().unwrap();
            let x :u32  = capture[2].parse().unwrap();
            let y :u32  = capture[3].parse().unwrap();
            let lenght :u32  = capture[4].parse().unwrap();
            let height :u32  = capture[5].parse().unwrap();
            claims.push(Claim::new(id,x,y,lenght,height));
        }
    }

    Ok(claims)
}

