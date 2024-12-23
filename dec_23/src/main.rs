use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn f_trs(g: & HashMap<String,HashSet<String>>) -> HashSet<[String;3]> {
    let mut trs = HashSet::<[String;3]>::new();
    for (v1, ns1) in g {
        for v2 in ns1 {
            for v3 in g[v2].iter() {
                if g[v3].contains(v1){
                    let mut t = [v1.clone(),v2.clone(),v3.clone()];
                    t.sort();
                    trs.insert(t);
                }
            }
        }
    }
    trs
}


fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("usage: program <file>");
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(&f);

    let mut g = HashMap::<String, HashSet<String>>::new();
    loop {
        let mut s = String::new();
        r.read_line(&mut s).expect("couldn't read file");
        s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        let [v1,v2] = s.split('-')
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .try_into().expect("couldn't split");
        if !g.contains_key(&v1) {
            g.insert(v1.clone(),HashSet::<String>::new());
        }
        if !g.contains_key(&v2) {
            g.insert(v2.clone(),HashSet::<String>::new());
        }
        g.get_mut(&v1).unwrap().insert(v2.clone());
        g.get_mut(&v2).unwrap().insert(v1.clone());
    }
    let trs = f_trs(&g);
    let mut sum = 0;
    for tr in trs {
        let mut has_t = false;
        for v in tr {
            if v.chars().nth(0).unwrap() == 't' {
                has_t = true;
                break;
            }
        }
        sum += if has_t {1} else {0};
    }
    println!("{:?}",sum);
}
