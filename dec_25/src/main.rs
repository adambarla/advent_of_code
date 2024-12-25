use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("usage: program <file>");
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(&f);

    let mut locks = Vec::<Vec<i32>>::new();
    let mut keys = Vec::<Vec<i32>>::new();
    let mut stop = false;
    'outer: loop {
        let mut scheme = Vec::<i32>::new();
        let mut lock = true;
        loop {
            let mut s = String::new();
            r.read_line(&mut s).expect("couldn't read line");
            s = s.trim().to_string();
            if s.is_empty() {
                if stop == true {
                    break 'outer;
                }
                stop = true;
                break;
            }
            stop = false;
            let tmp = s.chars().map(|c| if c == '#' {1} else {0}).collect::<Vec<i32>>();
            if scheme.is_empty(){
                for i in 0..tmp.len() {
                    if tmp[i] != 1 {
                        lock = false;
                        break;
                    }
                }
                scheme = tmp;
            }
            else{
                for i in 0..scheme.len() {
                    scheme[i] += tmp[i]
                }
            }
        }
        if lock {
            locks.push(scheme);
        }
        else {
            keys.push(scheme);
        }
    }
    // println!("{:?}", locks);
    // println!("{:?}", keys);
    let mut sum = 0;
    for l in locks.iter() {
        for k in keys.iter() {
            if l.len() != k.len() {
                continue;
            }
            let mut fits = true;
            for i in 0..l.len() {
                if l[i] + k[i] > 7 {
                    fits = false;
                    break;
                }
            }
            if !fits{
                continue
            }
            // println!("{:?} {:?}", l, k);
            sum += 1;
        }
    }
    println!("{}", sum);
}
