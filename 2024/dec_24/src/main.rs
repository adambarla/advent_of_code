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

fn fill(key: &String, gates: &HashMap<String, G>, vals: &mut HashMap<String, i32>, seen: &mut HashSet<String>) {
    if vals.contains_key(key) {
        return;
    }
    if !gates.contains_key(key){
        vals.insert(key.clone(),0);
        return;
    }
    if seen.contains(key) {
        vals.insert(key.clone(), i32::MAX);
        return;
    }
    seen.insert(key.clone());

    let g  = gates.get(key).unwrap();
    fill(&g.a, gates, vals, seen);
    fill(&g.b, gates, vals, seen);
    let v1 = vals[&g.a];
    let v2 = vals[&g.b];
    let res;
    match g.t {
        T::AND => res = v1 & v2,
        T::OR => res = v1 | v2,
        T::XOR => res = v1 ^ v2
    }
    vals.insert(key.clone(), res);
}

fn flag(key: &String, gates: &HashMap<String, G>, flagged: &mut HashSet<String>, seen: &mut HashSet<String>){
    if !gates.contains_key(key){
        return;
    }
    if seen.contains(key) {
        return;
    }
    seen.insert(key.clone());
    flagged.insert(key.clone());
    let g  = gates.get(key).unwrap();
    flag(&g.a,gates,flagged, seen);
    flag(&g.b,gates,flagged, seen);
}

fn test(gates: &HashMap<String,G>, n_bits: usize) -> (HashSet<String>, i32){
    let mut flagged = HashSet::<String>::new();
    let mut vals = HashMap::<String,i32>::new();
    let mut mistakes = 0;
    for i in 0..n_bits {
        for x in 0..=1 {
            for y in 0..=1 {
                vals.clear();
                let mut kx0 = "x".to_string();
                let mut ky0 = "y".to_string();
                let mut kz0 = "z".to_string();
                let mut kz1 = "z".to_string();
                kx0.push_str(format!("{:02}",i).as_str());
                ky0.push_str(format!("{:02}",i).as_str());
                kz0.push_str(format!("{:02}",i).as_str());
                kz1.push_str(format!("{:02}",i+1).as_str());
                vals.insert(kx0.clone(),x);
                vals.insert(ky0.clone(),y);
                fill(&kz0, &gates, &mut vals, &mut Default::default());
                fill(&kz1, &gates, &mut vals, &mut Default::default());
                if vals[&kz0] != (x + y) % 2 {
                    // println!("{} != {} ({} + {} % 2)", vals[&kz0], (x+y)%2, x, y);
                    mistakes += 1;
                    flag(&kz0, &gates, &mut flagged, &mut Default::default());
                }
                if vals[&kz1] != (x + y) / 2 {
                    // println!("{} != {} ({} + {} / 2)", vals[&kz1], (x+y)/2, x, y);
                    mistakes += 1;
                    flag(&kz1, &gates, &mut flagged, &mut Default::default());
                }
                // test carry
                if i == 0 {
                    continue;
                }
                vals.clear();
                let mut kx1 = "x".to_string();
                let mut ky1 = "y".to_string();
                kx1.push_str(format!("{:02}",i-1).as_str());
                ky1.push_str(format!("{:02}",i-1).as_str());
                vals.insert(kx0,x);
                vals.insert(ky0,y);
                vals.insert(kx1,1);
                vals.insert(ky1,1);
                fill(&kz0, &gates, &mut vals, &mut Default::default());
                fill(&kz1, &gates, &mut vals, &mut Default::default());
                if vals[&kz0] != (x + y + 1) % 2 {
                    // println!("{} != {} ({} + {} + 1 % 2)", vals[&kz0], (x+y+1)%2, x, y);
                    mistakes += 1;
                    flag(&kz0, &gates, &mut flagged, &mut Default::default());
                }
                if vals[&kz1] != (x + y + 1) / 2 {
                    // println!("{} != {} ({} + {} + 1 / 2)", vals[&kz1], (x+y+1)/2, x, y);
                    mistakes += 1;
                    flag(&kz1, &gates, &mut flagged, &mut Default::default());
                }
            }
        }
    }
    (flagged, mistakes)
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
    let mut xs = Vec::<String>::new();
    let mut ys = Vec::<String>::new();
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
        if key.contains('x') {
            xs.push(key.clone());
        }
        if key.contains('y') {
            ys.push(key.clone());
        }
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
        if key.contains('x') {
            xs.push(key.clone());
        }
        if key.contains('y') {
            ys.push(key.clone());
        }
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
    xs.sort();
    ys.sort();
    zs.sort();

    let mut zn = "".to_string();
    let mut n = 0u64;
    let mut k = zs.len() as u32;
    for z in zs.iter().rev() {
        k -= 1;
        fill(z, &gates, &mut vals, &mut Default::default());
        n += 2u64.pow(k) * vals[z] as u64;
        zn.push(vals[z].to_string().parse().unwrap());
    }
    println!("{:?}", zn);
    println!("{:?}", n);
    println!();

    // **
    let mut work = HashSet::<(String,String)>::new();
    let mut used = HashSet::<String>::new();
    // let mut sum = 0usize;
    loop {
        let (mut flagged, mistakes) = test(&gates, xs.len());
        let flagged = flagged.drain().collect::<Vec<_>>();
        println!("mistakes: {}, flagged: {}",mistakes, flagged.len());
        let mut best= ("".to_string(),"".to_string(),0);
        if mistakes == 0 {
            break;
        }
        for i in 0..flagged.len() {
            let k1 = &flagged[i];
            if used.contains(k1){
                continue;
            }
            for j in i+1..flagged.len() {
                let k2 = &flagged[j];
                if used.contains(k1){
                    break;
                }
                if used.contains(k2){
                    continue;
                }
                let g1 = gates.remove(k1).unwrap();
                let g2 = gates.remove(k2).unwrap();
                gates.insert(k1.clone(),g2);
                gates.insert(k2.clone(),g1);
                let (_,n_mistakes) = test(&gates, xs.len());
                if mistakes - n_mistakes > best.2 {
                    best.2 = mistakes - n_mistakes;
                    best.0 = k1.clone();
                    best.1 = k2.clone();
                    println!("{:?}",best);
                }
                let g2 = gates.remove(k1).unwrap();
                let g1 = gates.remove(k2).unwrap();
                gates.insert(k1.clone(),g1);
                gates.insert(k2.clone(),g2);
            }
        }
        let k1 = best.0;
        let k2 = best.1;
        let g1 = gates.remove(&k1).unwrap();
        let g2 = gates.remove(&k2).unwrap();
        gates.insert(k1.clone(),g2);
        gates.insert(k2.clone(),g1);
        work.insert((k1.clone(),k2.clone()));
        used.insert(k1);
        used.insert(k2);
        println!("{:?}",work);
    }
    let mut swapped = used.drain().collect::<Vec<_>>();
    swapped.sort();
    for (i,s) in swapped.iter().enumerate() {
        print!("{}",s);
        if i +1 != swapped.len() {
            print!(",");
        }
    }
    println!();
}
