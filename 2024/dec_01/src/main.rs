use std::io;
use std::collections::HashMap;

fn read(l1: &mut Vec<i32>,l2: &mut Vec<i32>) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let pair: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        l1.push(pair[0]);
        l2.push(pair[1]);
    }
}

fn distance(l1: &Vec<i32>,l2: &Vec<i32>) -> u32 {
    let mut dist = 0u32;
    for (a,b) in l1.iter().zip(l2.iter()) {
        dist += (a-b).abs() as u32;
    }
    return dist;
}


fn similarity(l1: &Vec<i32>,l2: &Vec<i32>) -> u32 {
    let mut map = HashMap::new();
    let mut sim = 0u32; 
    for i in 0..l2.len() {
        *map.entry(&l2[i]).or_insert(0) += 1;
    }
    for i in 0..l1.len() {
        sim += (l1[i] * map.get(&l1[i]).unwrap_or(&0)) as u32;
    }
    return sim;
}

fn main() {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    read(&mut l1, &mut l2);
    l1.sort();
    l2.sort();
    // let num = distance(&l1, &l2); // first *
    let num = similarity(&l1, &l2); // second * 
    println!("{}", num)
}

