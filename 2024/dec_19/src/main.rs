use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn fr(ps: &HashSet<String>, s: String, m: &mut HashMap<String,i64>) -> i64{
    if m.contains_key(&s) {
        return m[&s];
    }
    if s.is_empty() {
        return 1;
    }
    let mut c = 0;
    for p in ps {
        if p.len() > s.len(){ // order ps by size?
            continue;
        }
        let mut mtch = true;
        for i in 0..p.len(){
            if p.as_bytes()[i] != s.as_bytes()[i] {
                mtch = false;
                break
            }
        }
        if !mtch{
            continue;
        }
        c += fr(ps, s[p.len()..].parse().unwrap(), m);
    }
    m.insert(s,c);
    c
}

fn main() {
    let args = env::args();
    if args.len() != 2 {
        panic!("usage \"program <file>\"");
    }
    let f = File::open(&args.collect::<Vec<String>>()[1]).expect("couldn't open file");
    let mut r = BufReader::new(&f);

    let mut s = String::new();
    r.read_line(&mut s).expect("couldn't read line");
    s = s.trim().to_string();
    let p  = s.split(", ").map(|e| e.to_string()).collect::<HashSet<String>>();
    // println!("{:?}", p);
    r.read_line(&mut s).expect("couldn't read line");
    let mut m = HashMap::<String, i64>::new();
    let mut c = 0;
    loop {
        s.clear();
        r.read_line(&mut s).expect("couldn't read line");
        s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        c += fr(&p,s.clone(),&mut m);
    }
    println!("{}",c);
}
