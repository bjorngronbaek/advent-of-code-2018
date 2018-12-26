extern crate regex;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Task {
    pub id: String,
    pub before: String,
}

impl Task {

    pub fn new(id: String, before: String) -> Task {
        Task {
            id: id,
            before: before,
        }
    }
    
    pub fn parse(line: &str) -> Task {
        //Step F must be finished before step E can begin.
        let match_tasks = Regex::new(r"Step (\w{1}) must be finished before step (\w{1}) can begin").unwrap();
    
        let mut taskname = String::new();
        let mut before = String::new();
        for capture in match_tasks.captures_iter(line) {
            before = capture[1].to_string();
            taskname = capture[2].to_string();
        }

        Task::new(taskname,before)
    }

    pub fn get_all_ids(tasks: &Vec<Task>) -> Vec<String> {
        let mut ids: HashSet<String> = HashSet::new();
        for task in tasks {
            ids.insert(task.id.clone());
            ids.insert(task.before.clone());
        }

        let mut sorted_task_ids: Vec<String> = Vec::new();
        for id in &ids {
            sorted_task_ids.push(id.to_string());
        }

        sorted_task_ids
    }

    pub fn get_map(tasks: &Vec<Task>) -> HashMap<String,HashSet<String>> {
        let mut task_map: HashMap<String,HashSet<String>> = HashMap::new();
        for task in tasks {
            let entry = task_map.entry(task.id.clone()).or_insert(HashSet::new());
            entry.insert(task.before.clone());
        }

        task_map
    }
}