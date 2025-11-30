use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead};
use std::{env, io};

fn r(file: &str) -> Vec<Vec<char>> {
    let f = File::open(file).expect("Failed to open input file");
    let mut reader = io::BufReader::new(f);
    let mut gr = Vec::<Vec<char>>::new();
    loop {
        let mut line = String::new();
        // std::io::stdin().read_line(&mut line).unwrap();
        reader.read_line(&mut line).expect("Failed to read line");
        line = line.trim().to_string();
        if line.is_empty() {
            break;
        }
        let row: Vec<char> = line.chars().collect();
        gr.push(row);
    }
    gr
}

fn fnd(gr: &Vec<Vec<char>>) -> ((i32, i32),(i32,i32)) {
    let mut s = (0,0);
    let mut e = (0,0);
    for i in 0..gr.len() {
        for j in 0..gr[i].len() {
            if gr[i][j] == 'S' {
                s = (i as i32, j as i32);
            }
            if gr[i][j] == 'E' {
                e = (i as i32, j as i32);
            }
        }
    }
    (s,e)
}

#[derive(Eq, Copy, Clone)]
struct P {
    c: usize,
    p: (i32, i32),
    d: (i32, i32),
}

impl Ord for P {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.c.cmp(&self.c)
    }
}

impl PartialEq<Self> for P {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c
    }
}

impl PartialOrd for P {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Option::from(Ord::cmp(&other.c, &self.c))
    }
}

fn f(gr: &mut Vec<Vec<char>>, s: (i32, i32), e: (i32,i32)) -> HashMap<((i32,i32),(i32,i32)),Vec<((i32,i32),(i32,i32))>> {
    let mut q = BinaryHeap::<P>::new();
    let mut v = HashMap::<((i32,i32),(i32,i32)),usize>::new();
    let mut prev = HashMap::<((i32,i32),(i32,i32)),Vec<((i32,i32),(i32,i32))>>::new();
    q.push(P { c: 0, p: s, d: (0, 1) });
    v.insert((s, (0, 1)), 0);
    let mut smst = usize::MAX;
    while let Some(a) = q.pop() {
        if a.c > smst{
            continue
        }
        if a.p == e {
            smst = a.c;
        }
        let a0 = P{c: a.c + 1, p: (a.p.0 + a.d.0, a.p.1 + a.d.1), d: a.d};
        let a1 = P{c: a.c + 1000, p: (a.p.0, a.p.1), d: (-a.d.1,a.d.0)};
        let a2 = P{c: a.c + 1000, p: (a.p.0, a.p.1), d: (a.d.1,-a.d.0)};
        for d in vec![a0,a1,a2] {
            if gr[d.p.0 as usize][d.p.1 as usize] == '#' {
                continue;
            }
            if !v.contains_key(&(d.p, d.d)){
                v.insert((d.p, d.d), d.c);
                q.push(d);
                prev.insert((d.p,d.d), vec![(a.p,a.d)]);
            }
            else if let Some(&c) = v.get(&(d.p, d.d)) {
                if c > d.c {
                    v.insert((d.p, d.d), d.c);
                    q.push(d);
                    prev.insert((d.p,d.d), vec![(a.p,a.d)]);
                }
                if c == d.c {
                    prev.get_mut(&(d.p,d.d)).unwrap().push((a.p,a.d));
                }
            }
        }
    }
    prev
}

fn crwl(s: (i32, i32), e: (i32,i32), prev: &HashMap<((i32,i32),(i32,i32)),Vec<((i32,i32),(i32,i32))>>) -> usize {
    let mut v = HashSet::<(i32,i32)>::new();
    let mut q = VecDeque::<((i32,i32),(i32,i32))>::new();
    q.push_back((e,(0,1)));
    q.push_back((e,(1,0)));
    q.push_back((e,(0,-1)));
    q.push_back((e,(-1,0)));
    v.insert(e);
    while let Some((p,d)) = q.pop_front() {
        if p == s {
            continue;
        }
        if !prev.contains_key(&(p,d)) {
            continue;
        }
        for (np,nd) in prev[&(p,d)].iter() {
            v.insert(*np);
            q.push_back((*np,*nd));
        }
    }
    v.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <input file>", args[0]);
    }
    let mut gr = r(&args[1]);
    let (s, e) = fnd(&gr);
    let m = f(&mut gr, s, e);
    let c = crwl(s, e, &m);
    println!("{}", c);
}

