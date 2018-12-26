extern crate aoc_helper;
extern crate aoc07;
use std::env;
use std::process;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc_helper::Config;
use aoc07::Task;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    let entries = aoc_helper::run(config).unwrap();
    let mut tasks: Vec<Task> = Vec::new();
    for entry in entries {
        let mut task = Task::parse(&entry);        
        tasks.push(task);
    }

    let sorted_tasks = Task::get_all_ids(&tasks);
    let task_map = Task::get_map(&tasks);

    let mut task_order: HashSet<String> = HashSet::new();
    for task in sorted_tasks {
        match task_map.get(&task) {
            None => {task_order.insert(task);},
            Some(befores) => {
                println!("{} depends on {:?}",task,befores);
                let mut all_befores_are_done = true;
                for b in befores {
                    if !task_order.contains(b) {
                        all_befores_are_done = false;
                    }
                }
                if all_befores_are_done {
                    task_order.insert(task);
                }
            }
        }        
    }

    for task in task_order {
        println!("{}",task);
    }
}