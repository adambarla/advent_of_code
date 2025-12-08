use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::min;

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

fn dist(a : (i64,i64,i64), b: (i64,i64,i64)) -> i64 {
    return (a.0-b.0).pow(2) + (a.1-b.1).pow(2) + (a.2 - b.2).pow(2);
}

#[derive(Clone, Copy,Debug,PartialEq,Eq)]
struct S {
    dst : i64,
    pts : (usize, usize),
}


impl Ord for S {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dst.cmp(&self.dst)
    }

}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file = get_file();
    let mut points = Vec::<(i64,i64,i64)>::new();
    for l in file.split('\n') {
        let res : Result<Vec::<i64>, _> = l.split(',').map(|x| x.parse::<i64>()).collect();
        let point = match res {
            Ok(nums) => (nums[0],nums[1],nums[2]),
            Err(_) => break,
        };
        points.push(point);
    }
    // compute dis
    let mut q = BinaryHeap::<S>::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let dst = dist(points[i], points[j]);
            q.push(S { dst, pts: (i,j)});
        }
    }
    // println!("{:?}",q);
    let mut e: HashMap<usize,HashSet<usize>> = HashMap::new();
    let mut g_assign : Vec::<usize> = Vec::new();
    for i in 0..points.len() {
        g_assign.push(i);
    }
    println!("{:?}",points.len());
    let n = if points.len() == 20 {10} else {1000};
    let mut count = 0;
    loop {
        if q.is_empty() || count >= n {
            break;
        }
        let s = q.pop().unwrap();
        let (i,j) = s.pts;
        println!("connectiong {:?} with dst {}", s.pts, s.dst);
        if !e.contains_key(&i) {
            e.insert(i, HashSet::new());
        }
        if !e.contains_key(&j) {
            e.insert(j, HashSet::new());
        }
        if g_assign[i] != g_assign[j] {
            let g_i = g_assign[i];
            let g_j = g_assign[j];
            let min_g = min(g_i,g_j);
            for k in 0..g_assign.len() {
                if g_assign[k] == g_i || g_assign[k] == g_j {
                    g_assign[k] = min_g;
                }
            }
            e.get_mut(&i).unwrap().insert(j);
            e.get_mut(&j).unwrap().insert(i);
        }
        count += 1;
    }
    println!("{:?}",g_assign);
    // collect groups
    let mut g: Vec<usize> = vec![0;points.len()];
    for i in 0..g_assign.len() {
        g[g_assign[i]] += 1;
    }
    g.sort();
    println!("{:?}",g);
    let mut prod = 1;
    for i in g.len()-3..g.len() {
        println!("{} ", g[i]);
        prod *= g[i];
    }
    println!("{:?}",prod);
}
