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

fn dfs(g: &HashMap<String, Vec<String>>, start: &String, end: &String, path: &mut Vec<String>, paths: &mut HashSet<Vec<String>>) {
    if start == end {
        paths.insert(path.clone());
        return;
    }
    if !g.contains_key(start) {
        return;
    }
    for neighbor in g.get(start).unwrap() {
        // add neighbor to path
        if path.contains(neighbor) {
            panic!("Path contains neighbor: {:?}", path);
        }
        path.push(neighbor.clone());
        dfs(g, neighbor, end, path, paths);
        // remove last k characters (lenght of neighbor)
        path.pop();
    }
}

// find all paths from start to end
fn find_paths(g: &HashMap<usize, Vec<usize>>, start: usize, end: usize, n_vs: usize, reachable_from_end: &HashSet<usize>) -> Vec<i128> {
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(start);
    let mut counts: Vec<i128> = vec![0; n_vs];
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        if current == end {
            continue;
        }
        if !g.contains_key(&current) {
            continue;
        }
        for neighbor in g.get(&current).unwrap() {
            counts[*neighbor] += 1;
            if !reachable_from_end.contains(neighbor) {
                continue;
            }
            q.push_back(*neighbor);
        }
    }
    return counts;
}

fn get_rechanle_from_end(rev_g: &HashMap<usize, Vec<usize>>, end: usize) -> HashSet<usize> {
    let mut reachable_from_end: HashSet<usize> = HashSet::new();
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(end);
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        if !rev_g.contains_key(&current) {
            continue;
        }
        for neighbor in rev_g.get(&current).unwrap() {
            if reachable_from_end.contains(neighbor) {
                continue;
            }
            reachable_from_end.insert(*neighbor);
            q.push_back(*neighbor);
        }
    }
    return reachable_from_end;
}

fn main() {
    let file = get_file();
    let mut g_str: HashMap<String, Vec<String>> = HashMap::new();
    let mut vs: HashSet<String> = HashSet::new();
    for line in file.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let key = parts[0].to_string();
        let values: Vec<&str> = parts[1].trim().split(" ").collect();
        g_str.insert(key.clone(), values.iter().map(|s| s.to_string()).collect());
        vs.insert(key.clone());
        for value in values {
            vs.insert(value.to_string());
        }
    }
    let mut map: HashMap<String, usize> = HashMap::new();
    for (i, v) in vs.iter().enumerate() {
        map.insert(v.clone(), i);
    }
    let mut g: HashMap<usize, Vec<usize>> = HashMap::new();
    for (vertex, neigh) in g_str.iter() {
        let k_idx = *map.get(vertex).unwrap();
        for neighbor in neigh {
            let v_idx = *map.get(neighbor).unwrap();
            g.entry(k_idx).or_insert(Vec::new()).push(v_idx);
        }
    }
    let mut rev_g: HashMap<usize, Vec<usize>> = HashMap::new();
    for (vertex, neigh) in g.iter() {
        for neighbor in neigh {
            rev_g.entry(*neighbor).or_insert(Vec::new()).push(*vertex);
        }
    }

    // println!("{:?}", g);
    // println!("{:?}", rev_g);
    // println!("{:?}", vs);
    // println!("number of vertices: {}", vs.len());

    // part 1
    let start_str = "you";
    let end_str = "out";
    if map.contains_key(start_str) && map.contains_key(end_str) {
        let start = *map.get(start_str).unwrap();
        let end = *map.get(end_str).unwrap();
        let rechable = get_rechanle_from_end(&rev_g, end);
        let counts = find_paths(&g, start, end, vs.len(), &rechable);
        println!("part 1: {}", counts[end]);
    }

    // part 2
    let combinations = [
        ["svr", "dac", "fft", "out"], 
        ["svr", "fft", "dac","out"],
    ];
    let mut count = 0;
    'comb_loop:for combination in combinations {
        println!("combination: {:?}", combination);
        // check all keys in combination are in map
        let mut ids: Vec<usize> = Vec::new();
        for v in combination {
            if !map.contains_key(v) {
                panic!("key `{}` not found in map", v);
            }
            ids.push(*map.get(v).unwrap());
        }
        // check if combination is valid (all consecutive vertices are connected)
        for i in 0..combination.len() - 1 {
            let start = ids[i];
            let end = ids[i + 1];
            let rechable = get_rechanle_from_end(&rev_g, end);
            if !rechable.contains(&start) {
                println!("\t{} -> {} : 0 paths", combination[i], combination[i + 1]);
                println!("\tNO PATH");
                continue 'comb_loop;
            }
        }
        let mut product = 1;
        for i in 0..combination.len() - 1 {
            let start = ids[i];
            let end = ids[i + 1];
            let rechable = get_rechanle_from_end(&rev_g, end);
            let counts = find_paths(&g, start, end, vs.len(), &rechable);
            println!("\t{} -> {} : {} paths", combination[i], combination[i + 1], counts[end]);
            product *= counts[end];
        }
        println!("\tproduct: {}", product);
        count += product;
    }
    println!("number of paths for all combinations: {}", count);
}
