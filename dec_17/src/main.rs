use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn combo(rs: &mut Vec<i32>, o: i32) -> i32 {
    match o {
        0..=3 => {o}
        4..=6 => {rs[(o-4) as usize]}
        7 => {panic!("reserved operand")}
        _ => {panic!("invalid operand")}
    }
}

fn instr(rs: &mut Vec<i32>, ptr: &mut i32, i: i32, o: i32) -> Option<i32> {
    println!("instruction {i} operand {o}");
    match i {
        0 => {rs[0] = rs[0] / 2_i32.pow(combo(rs, o) as u32)}
        1 => {rs[1] = rs[1] ^ o}
        2 => {rs[1] = combo(rs,o) % 8}
        3 => {if rs[0] != 0 {*ptr = o - 2}}
        4 => {rs[1]  = rs[1] ^ rs[2]}
        5 => {return Some(combo(rs, o) % 8) }
        6 => {rs[1] = rs[0] / 2_i32.pow(combo(rs, o) as u32)}
        7 => {rs[2] = rs[0] / 2_i32.pow(combo(rs, o) as u32)}
        _ => {panic!("invalid operand")}
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage \"cargo run <file>\"")
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(f);

    let m = 8;
    let mut rs: Vec<i32> = vec![0, 0, 0];
    let mut s = String::new();
    for i in 0..3 {
        s.clear();
        r.read_line(&mut s).expect("couldn't read line");
        s = s.trim().to_string();
        rs[i] = s
            .split(":")
            .skip(1)
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>()[0];
    }
    s.clear();
    r.read_line(&mut s).expect("couldn't read line");
    r.read_line(&mut s).expect("couldn't read line");
    s = s.split(":").skip(1).collect::<Vec<_>>()[0].trim().to_string();
    let mut ins: Vec<i32> = s.split(",").map(|n| n.parse().unwrap()).collect();
    let mut ptr :i32 = 0;
    let mut v = Vec::<i32>::new();
    loop {
        if ptr >= ins.len() as i32 {
            break;
        }
        println!("reg {:?}", rs);
        println!("ins {:?}", ins);
        println!("ptr {:?}", ptr);
        let inst = ins[ptr as usize];
        let op = ins[(ptr+1) as usize];
        let out = instr(&mut rs, &mut ptr, inst, op);
        if !out.is_none() {
            // print!("{},",out.unwrap())
            v.push(out.unwrap());
        }
        ptr += 2;
    }
    println!("reg {:?}", rs);
    println!("ins {:?}", ins);
    println!("ptr {:?}", ptr);
    for n in v {
        print!("{},",n);
    }
    println!();
}
