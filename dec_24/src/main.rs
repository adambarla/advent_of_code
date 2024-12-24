use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, Hash, PartialEq, Debug)]
enum T {
    AND,
    OR,
    XOR,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct G {
    a: String,
    b: String,
    t: T,
}

fn fill(key: &String, gates: &HashMap<String, G>, vals: &mut HashMap<String, i32>) {
    if vals.contains_key(key) {
        return;
    }
    let g  = gates.get(key).unwrap();
    fill(&g.a, gates, vals);
    fill(&g.b, gates, vals);
    let v1 = vals[&g.a];
    let v2 = vals[&g.b];
    let mut res;
    match g.t {
        T::AND => res = v1 & v2,
        T::OR => res = v1 | v2,
        T::XOR => res = v1 ^ v2
    }
    vals.insert(key.clone(), res);
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("usage: program <file>");
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(&f);

    let mut gates = HashMap::<String, G>::new();
    let mut vals = HashMap::<String, i32>::new();
    let mut zs = Vec::<String>::new();
    loop {
        let mut s = String::new();
        r.read_line(&mut s).expect("couldn't read file");
        s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        let kv: Vec<_> = s.split(':').collect();
        let key = kv[0].to_string();
        let val: i32 = kv[1].trim().parse().expect("couldn't parse");
        if key.contains('z') {
            zs.push(key.clone());
        }
        vals.insert(key, val);
    }
    loop {
        let mut s = String::new();
        r.read_line(&mut s).expect("couldn't read file");
        s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        let values: Result<[String; 5], _> = s
            .split(' ')
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .try_into();
        let (a, t, b, _, key) = values.unwrap().try_into().expect("couldn't unwrap");
        if key.contains('z') {
            zs.push(key.clone());
        }
        gates.insert(
            key,
            G {
                a,
                b,
                t: if t == "AND" {
                    T::AND
                } else {
                    if t == "OR" {
                        T::OR
                    } else {
                        T::XOR
                    }
                },
            },
        );
    }
    println!("{:?}", gates);
    zs.sort();
    println!("{:?}", zs);
    let mut zn = "".to_string();
    let mut n = 0u64;
    let mut k = zs.len() as u32;
    for z in zs.iter().rev() {
        k -= 1;
        fill(z, &gates, &mut vals);
        n += 2u64.pow(k) * vals[z] as u64;
        zn.push(vals[z].to_string().parse().unwrap());
    }

    println!("{:?}", zn);
    println!("{:?}", n);
}
