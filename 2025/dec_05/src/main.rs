use std::fs;
use std::env;
use std::collections;

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
    let mut ranges = Vec::<(i64, i64)>::new();
    // read ranges
    let lines = file.split('\n').collect::<Vec<&str>>();
    let mut i = 0;
    loop {
        let line = &lines[i];
        let res: Result<Vec<i64>, _> = line.split('-').map(|r| r.parse::<i64>()).collect();
        let r = match res {
            Ok(v) => (v[0], v[1]),
            Err(_) => break,
        };
        ranges.push(r);
        i += 1;
    }
    i += 1;
    // println!("{:?}",ranges);
    // read input
    let mut prompts = Vec::<i64>::new();
    loop {
        let line = &lines[i];
        let res: Result<i64, _> = line.parse::<i64>();
        let x = match res {
            Ok(num) => num,
            Err(_) => break,
        };
        prompts.push(x);
        i += 1;
    }
    // println!("{:?}",prompts);
    // get fresh
    let mut fresh = 0;
    for p in prompts {
        for r in &ranges {
            if p >= r.0 && p <= r.1 {
                fresh += 1;
                break;
            }

        }
    }
    println!("fresh: {}",fresh);
    // get fresh IDs
    loop {
        let mut overlaps = 0;
        for i in 0..ranges.len() {
            for j in i+1..ranges.len() {
                // check overlap
                let r1 = &ranges[i];
                let r2 = &ranges[j];
                if r1.1 >= r2.0 && r1.0 <= r2.g1 {

                }
            }
        }
        if overlaps == 0 {
            break;
        }
    }
}
