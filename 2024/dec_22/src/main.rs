use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("usage: program <file>")
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(&f);
    let mut ns = Vec::<u32>::new();
    loop {
        let mut s = String::new();
        r.read_line(&mut s).expect("couldn't read line");
        s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        ns.push(s.parse::<u32>().expect("couldn't parse"));
    }

    let m = 16777216;
    let k = 2000;
    let mut sum = 0u64;
    let mut seqs = HashMap::<[i32; 4], u32>::new();
    for i in 0..ns.len() {
        let mut prev = ns[i] % 10;
        let mut q = VecDeque::<i32>::new();
        let mut part_seqs = HashMap::<[i32; 4], u32>::new();
        for j in 0..k {
            let mut a = ns[i];
            a <<= 6;
            a ^= ns[i];
            a %= m;
            ns[i] = a;
            a >>= 5;
            a ^= ns[i];
            a %= m;
            ns[i] = a;
            a <<= 11;
            a ^= ns[i];
            a %= m;
            ns[i] = a;
            let tmp = ns[i] % 10;
            q.push_back(tmp as i32 - prev as i32);
            prev = tmp;
            if j <= 3 {
                continue;
            }
            q.pop_front();
            let v: [i32; 4] = q.make_contiguous().try_into().expect("couldn't make array");
            if part_seqs.contains_key(&v) {
                continue;
            }
            part_seqs.insert(v, tmp);
        }
        for (key, val) in part_seqs {
            if !seqs.contains_key(&key) {
                seqs.insert(key, val);
                continue;
            }
            *seqs.get_mut(&key).unwrap() += val;
        }
        sum += ns[i] as u64;
    }
    let mut max = 0;
    for (_, val) in seqs {
        if val > max {
            max = val;
        }
    }
    println!("{}", sum);
    println!("{:?}", max);
}
