extern crate aoc_helper;
extern crate aoc06;
use std::env;
use std::process;
use std::collections::HashMap;

use aoc_helper::Config;
use aoc06::Point;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut points: Vec<Point> = Vec::new();
    let entries = aoc_helper::run(config).unwrap();
    for entry in entries {
        points.push(Point::parse(&entry));
    }

    for point in &points {
        println!("Point: ({},{})",point.x,point.y);
    }

    let (tl,br) = Point::find_borders(&points);
    println!("{},{} {},{}",tl.x,tl.y,br.x,br.y);

    let mut owner_map: HashMap<Point,Point> = HashMap::new;
    for i in tl.x+1..br.x-1 {
        for j in tl.y+1..br.y-1 {
            let mut closest_dist = std::usize::MAX;
            for point in &points{
                let dist = point.m_distance(new::Point(i,j));
                if dist < closest_dist {
                    closest_dist = dist;
                    let owner = owner_map.insert(Point::new(i,j),p);
                }
            }            
        }
    }
}