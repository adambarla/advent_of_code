use std::env;
use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

fn get_file() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide input file path as an argument!")
    }
    let file_path = &args[1];
    let file = fs::read_to_string(file_path);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            panic!("Error when reading file: {}", e)
        }
    };
    return file;
}


fn main() {
    let file = get_file();
    let mut g: HashMap<String, Vec<String>> = HashMap::new();
    let mut vs: HashSet<String> = HashSet::new();
    for line in file.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let key = parts[0].to_string();
        let values: Vec<&str> = parts[1].trim().split(" ").collect();
        g.insert(key.clone(), values.iter().map(|s| s.to_string()).collect());
        vs.insert(key.clone());
        for value in values {
            vs.insert(value.to_string());
        }
    }
    // println!("{:?}", g);
    // println!("{:?}", vs);

    let start = "you";
    let end = "out";
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(start.to_string());
    let mut counts: HashMap<String, i32> = HashMap::new();
    for v in vs {
        counts.insert(v.clone(), 0);
    }
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        if current == end {
            continue;
        }
        for neighbor in g.get(&current).unwrap() {
            let count = counts.get_mut(neighbor).unwrap();
            *count += 1;
            q.push_back(neighbor.clone());
        }
    }
    // println!("{:?}", counts);
    println!("part 1: {}", counts.get(end).unwrap());

}
