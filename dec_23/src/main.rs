use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Prefix::DeviceNS;

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

fn is_clq(g: &HashMap<String,HashSet<String>>, vs: &Vec<String>) -> bool {
    for i in 0..vs.len(){
        for j in i+1..vs.len(){
            if !g[&vs[i]].contains(&vs[j]) {
                return false;
            }
        }
    }
    true
}

fn f_mclq(g: &HashMap<String,HashSet<String>>, vs: &Vec<String>, subset: &mut Vec<String>, i: usize, k: &mut f64, best: &mut usize) -> Vec<String> {
    if !is_clq(g,subset){
        *k += 2_f64.powi((vs.len()-i).try_into().unwrap());
        println!("{:10}", *k / 2_f64.powi(vs.len() as i32));
        return Default::default();
    }
    if i == vs.len() {
        *k += 1f64;
        *best = subset.len();
        return subset.clone();
    }
    if subset.len() + (vs.len() - i - 1) <= *best{
        return Default::default();
    }
    let s1 = f_mclq(g,vs,subset, i+1, k, best);
    subset.push(vs[i].clone());
    let s2 = f_mclq(g,vs,subset, i + 1, k, best);
    subset.pop();
    if s1.len() >= s2.len() {
        return s1;
    }
    s2
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

    let mut vs = Vec::<String>::new();
    for (v,_) in g.iter() {
        vs.push(v.clone());
    }
    let mut clq = f_mclq(&g, &vs, &mut Default::default(),0,&mut 0f64, &mut 0);
    clq.sort();
    for i in 0..clq.len() {
        print!("{}",clq[i]);
        if i != clq.len() -1 {
            print!(",");
        }
    }
    println!();

}
