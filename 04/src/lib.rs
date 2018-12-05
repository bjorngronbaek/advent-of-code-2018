extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;
use std::error::Error;
use std::collections::HashMap;
use std::fmt;


pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub struct Sleep {
    pub minutes: [u32;60],
    pub total : usize,
}

impl Sleep {
    pub fn new(sleep_start: usize, sleep_end: usize) -> Sleep {
        let mut m: [u32;60] = [0; 60];
        
        for i in sleep_start..sleep_end {
            m[i] = m[i]+1;
        }

        Sleep {
            minutes: m,
            total: sleep_end - sleep_start,
        }
    }

    pub fn add_sleep(&mut self,sleep_start: usize, sleep_end: usize) {
        for i in sleep_start..sleep_end {
            self.minutes[i] = self.minutes[i]+1;
        }

        self.total += sleep_end - sleep_start;
    }

    pub fn max_minute(&self) -> (usize,u32) {
        let mut max_minute = 0;
        let mut max_observed = 0;
        for i in 0..59 {
            if &self.minutes[i] >= &max_observed {
                max_observed = self.minutes[i];
                max_minute = i;
            }
        }
        
        (max_minute,max_observed)
    }
}

impl fmt::Display for Sleep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.minutes.iter() {            
            write!(f, "{},", i)?;        
        }

        write!(f,"\ttotal: {}",self.total);

        Ok(())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let reader = io::BufReader::new(file);
    
    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        entries.push(line);
    }

    entries.sort();
    //[1518-11-01 00:00] Guard #10 begins shift
    let match_guard = Regex::new(r"Guard #(\d+)").unwrap();
    let match_event = Regex::new(r"^\[\d{4}-\d{2}-\d{2}\s\d{2}:(\d{2})\]\s(\w+\s\w+)").unwrap();
    let mut id :u32 = 0;
    let mut minute_sleep_start :usize = 0;
    let mut minute_sleep_end :usize = 0;
    let mut sleeping = false;

    let mut guard_map = HashMap::new();
    for entry in entries {
        if match_guard.is_match(&entry) {
            println!("Found new guard!");
            if sleeping {
                println!("old guard {} is sleeping!",id);
                //new guard, but last one didn't awake. Set minute to 60
                sleeping = false;
                let sleep = guard_map.entry(id).or_insert(Sleep::new(minute_sleep_start,60));
                sleep.add_sleep(minute_sleep_start,60)
            }

            id = match_guard.captures(&entry).unwrap().get(1).unwrap().as_str().parse().unwrap();            
            println!("new guard is: {}",&id);
            sleeping = false;
        }
        else {
            let event = match_event.captures(&entry).unwrap().get(2).unwrap().as_str();
            let minute = match_event.captures(&entry).unwrap().get(1).unwrap().as_str().parse().unwrap();

            if event == "falls asleep" {
                println!("guard falls asleep at: {}",minute);
                sleeping = true;
                minute_sleep_start = minute;
            }
            else {
                println!("guard wakes up at: {}",minute);
                sleeping = false;
                //let sleep = guard_map.entry(id).or_insert(Sleep::new(minute_sleep_start,minute));
                let sleep = guard_map.entry(id).or_insert(Sleep::new(0,0));
                sleep.add_sleep(minute_sleep_start,minute)

            }            
        }            
    }

    let mut max = 0;
    let mut most_sleeping = 0;
    for (id,sleep) in &guard_map {
        let (mm,ms) = sleep.max_minute();
        println!("{}:\t {}\t highest minute: {}\t highest value: {}",id,sleep,mm,ms);
        if sleep.total >= max {
            max = sleep.total;
            most_sleeping = *id;
        }
    }    

    if let Some(ref sleep) = guard_map.get(&most_sleeping){
        let mut max_minute = 0;
        let mut max_observed = 0;
        for i in 0..59 {
            if sleep.minutes[i] >= max_observed {
                max_observed = sleep.minutes[i];
                max_minute = i;
            }
        }    
        println!("Most sleeping guard: {} slept {} minues, {} times at minute {}",most_sleeping,max,max_observed,max_minute);
    }

    Ok(())
}

/*
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
*/