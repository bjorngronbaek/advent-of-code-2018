extern crate aoc_helper;
extern crate aoc06;
use std::env;
use std::process;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc_helper::Config;
use aoc06::Point;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let entries = aoc_helper::run(config).unwrap();
    let mut owner_map: HashMap<Point,usize> = HashMap::new();
    let mut owner_set: HashSet<Point> = HashSet::new();
    for entry in entries {
        owner_map.insert(Point::parse(&entry),0);
        owner_set.insert(Point::parse(&entry));
    }

    let (tl,br) = Point::find_borders(&owner_set);
    println!("Borders are: {:?} - {:?}",tl,br);

    let mut border_points : HashSet<Point> = HashSet::new();

    //calculate ownership of all coordinates within border and update the count
    let mut distance_map: HashMap<Point,usize> = HashMap::new();
    for i in tl.x..br.x+1 {
        for j in tl.y..br.y+1 {
            let target = Point::new(i,j);
            let point = target.find_closest_point(&owner_set);
            let distance = target.find_total_distance(&owner_set);
            distance_map.insert(target,distance);

            match point {
                Some(p) => {
                    if i == tl.x || i == br.x || j == tl.y || j == br.y {
                        //println!("Found closes to border at ({},{}):\t{:?}",i,j,p);
                        border_points.insert(p);
                    }
                    else {
                        *owner_map.get_mut(&p).unwrap() += 1;
                    }
                },
                None => {}
            }
        }
    }

    //if the point owns a border coordinate, remove it!
    for p in &border_points {
        //println!("{:?} has ownership on the border",p);
        owner_map.remove(p);
    }

    let mut max_area = std::usize::MIN;
    let mut max_owner = &Point::new(0,0);
    for (point,count) in &owner_map {
        //println!("({:?}); \t{}",point,count);
        if *count > max_area {
            max_owner = &point;
            max_area = *count;
        }
    }
    println!("Max point is: {:?} with {}",max_owner,max_area);

    let mut size = 0;
    for (point,distance) in &distance_map {        
        if *distance < 10000 {
            size += 1;
        }
    }
    println!("Size of region: {}",size);


}