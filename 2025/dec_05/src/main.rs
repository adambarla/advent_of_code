use std::collections::HashSet;
use std::fs;
use std::env;

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
    let mut ranges = HashSet::<(i128,i128)>::new();
    // read ranges
    let lines = file.split('\n').collect::<Vec<&str>>();
    let mut i = 0;
    loop {
        let line = &lines[i];
        let res: Result<Vec<i128>, _> = line.split('-').map(|r| r.parse::<i128>()).collect();
        let r = match res {
            Ok(v) => (v[0], v[1]),
            Err(_) => break,
        };
        ranges.insert(r);
        i += 1;
    }
    i += 1;
    println!("{:?}",ranges);
    // read input
    let mut prompts = Vec::<i128>::new();
    loop {
        let line = &lines[i];
        let res: Result<i128, _> = line.parse::<i128>();
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
        let mut toremove = HashSet::<(i128,i128)>::new();
        let mut toinsert = HashSet::<(i128,i128)>::new();
        let mut it1 = ranges.iter();
        loop {
            let mut it2 = it1.clone();
            it2.next();
            let r1 = match it1.next() {
                None => break,
                Some(r) => r,
            };
            if toremove.contains(r1) {
                continue;
            }
            
            loop {
                // check overlap
                let r2= match it2.next() {
                    None => break,
                    Some(r) => r,
                };

                if toremove.contains(r2) || toremove.contains(r1){
                    continue;
                }

                if r1.1 >= r2.0 && r1.0 <= r2.1 {
                    let l = std::cmp::min(r1.0, r2.0);
                    let h = std::cmp::max(r1.1, r2.1);
                    toremove.insert(*r2);
                    toremove.insert(*r1);
                    toinsert.insert((l,h));
                    overlaps += 1;
                }
            }
        }
        for r in toremove {
            ranges.remove(&r);
        }
        for r in toinsert {
            ranges.insert(r);
        }
        println!("{:?}",ranges);
        if overlaps == 0 {
            break;
        }
    }
    // sum the lengths
    let mut num_ids: i128 = 0;
    for r in ranges {
        let mut l = r.1 - r.0;
        l += 1;
        // if r.0 != r.1 {
        //     l += 1;
        // }
        num_ids += l as i128;
    }
    println!("num ids: {}",num_ids);
}
