use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn valid(m: &Vec<Vec<u8>>, b: &(i32,i32)) -> bool {
   b.0 >= 0 && b.0 < m.len() as i32 && b.1 >= 0 && b.1 < m[0].len() as i32
       && m[b.0 as usize][b.1 as usize] as char != '#'
}

fn bfs(m: &Vec<Vec<u8>>, s: (i32, i32)) -> HashMap::<(i32,i32),i32>  {
    let mut v = HashMap::<(i32, i32), i32>::new();
    let mut q = VecDeque::<(i32, i32)>::new();

    q.push_back(s);
    v.insert(s,0);

    while let Some(a) = q.pop_front() {
        for (di,dj) in [(0,1), (1,0), (0,-1), (-1,0)] {
            let b = (a.0 + di, a.1 + dj);
            if v.contains_key(&b) || !valid(m,&b) {
                continue;
            }
            q.push_back(b);
            v.insert(b,v[&a] + 1);
        }
    }
    v
}

fn sh(m: &Vec<Vec<u8>>, p: (i32,i32), r: i32) -> Vec<((i32,i32),i32)> {
    let mut s = HashSet::<((i32,i32),i32)>::new();
    for i in 0..=r{
        for j in 0..=r-i {
            for (di,dj) in [(i,j),(-i,j),(i,-j),(-i,-j)]{
                if i + j < 2 {
                    continue
                }
                let q = (p.0 + di, p.1 + dj);
                if valid(m, &q){
                    s.insert((q,i+j));
                }
            }
        }
    }
    s.drain().collect()
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("usage: program <file>");
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(&f);
    let mut m = Vec::<Vec<u8>>::new();
    loop {
        let mut s = String::new();
        r.read_line(&mut s).expect("couldn't read line");
        s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        m.push(Vec::from(s.as_bytes()))
    }
    let mut s = (0,0);
    let mut e = (0,0);
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] as char == 'E'{
                e = (i as i32,j as i32);
            }
            else if m[i][j] as char == 'S' {
                s = (i as i32,j as i32);
            }
            // print!("{}", m[i][j] as char);
        }
        // println!();
    }
    let fs = bfs(&m,s);
    let te = bfs(&m,e);
    let og = fs[&e];
    let r= 20;
    let tol = 100;
    let mut c = vec![0; (og + 1) as usize];
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] as char == '#'{
                continue
            }
            let p = (i as i32, j as i32);
            let shorts = sh(&m, p, r);
            // println!("({},{}) -> {:?}",i,j,shorts);
            for (st, sl) in shorts{
                let new = fs[&p] + te[&st] + sl;
                if og - new >= 0 {
                   c[(og - new) as usize] += 1;
                // println!("({},{}) -> {}",i,j,og - new);
                }
            }
        }
    }
    let mut sum = 0;
    for i in tol..og+1{
        sum += c[i as usize];
        // println!("{}: {}",i, c[i as usize]);
    }
    println!("{}",sum);

}
