use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

fn c(rs: &[u64; 3], o: u64) -> u64 {
    match o {
        0..=3 => {o}
        4..=6 => {rs[(o-4) as usize]}
        7 => {panic!("reserved operand")}
        _ => {panic!("invalid operand")}
    }
}

fn instr(r: &mut [u64; 3], p: &mut u64, i: u64, o: u64) -> Option<u64> {
    let mut res = None;
    match i {
        0 => {r[0] = r[0] / 2_u64.pow(c(r, o) as u32)}
        1 => {r[1] = r[1] ^ o}
        2 => {r[1] = c(r,o) % 8}
        3 => {if r[0] != 0 {*p = o}}
        4 => {r[1]  = r[1] ^ r[2]}
        5 => {res =  Some(c(r, o) % 8) }
        6 => {r[1] = r[0] / 2_u64.pow(c(r, o) as u32)}
        7 => {r[2] = r[0] / 2_u64.pow(c(r, o) as u32)}
        _ => {panic!("invalid operand")}
    }
    if i != 3 || r[0] == 0 {
        *p += 2;
    }
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage \"cargo run <file>\"")
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(f);

    let mut reg_og: [u64;3] = [0;3];
    let mut s = String::new();
    for i in 0..3 {
        s.clear();
        r.read_line(&mut s).expect("couldn't read line");
        s = s.trim().to_string();
        reg_og[i] = s
            .split(":")
            .skip(1)
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>()[0];
    }
    s.clear();
    r.read_line(&mut s).expect("couldn't read line");
    r.read_line(&mut s).expect("couldn't read line");
    s = s.split(":").skip(1).collect::<Vec<_>>()[0]
        .trim()
        .to_string();
    let ins: Vec<u64> = s.split(",").map(|n| n.parse().unwrap()).collect();

    let mut a =  0o133267275; // the digits in octal rep. of a don't change when it matches first few outputs
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut best_i = 0;
    loop {
        let mut ptr: u64 = 0;
        let mut found = true;
        let mut i = 0;
        let mut reg = reg_og.clone();
        reg[0] = a;
        loop {
            if ptr as usize >= ins.len() {
                break;
            }
            // println!("reg {:?}", reg);
            // println!("ins {:?}", ins);
            // println!("ptr {:?}", ptr);
            let inst = ins[ptr as usize];
            let op = ins[(ptr + 1) as usize];
            let out = instr(&mut reg, &mut ptr, inst, op);
            if !out.is_none() {
                // println!("{}",out.unwrap());
                if out.unwrap() != ins[i] {
                    found = false;
                    if i > best_i {
                        best_i = i;
                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        // println!("done {:.10}% s {:.2}", a as f64 / u64::MAX as f64, (end - start).as_secs());
                        println!("{:o}: {}", a, i);
                    }
                    break;
                }
                i += 1;
            }
        }
        if found && i == ins.len() {
            break;
        }
        a += 0o1000000000; // search the rest
    }
    println!("{}", a);
    // for n in v {
    //     print!("{},",n);
    // }
    // println!();
}

// attempt at a recursive solution
// fn f_r(ins: &mut Vec<u64>, mut reg: [u64;3], ptr: u64, p_i: u64) -> u64 {
//     let o = ins[ptr as usize +1];
//     let i = ins[ptr as usize];
//     match i {
//         0 => {
//             if reg[0] != u64::MAX {
//                 if o == 4 {
//                     if reg[0] != 0 {
//                         return u64::MAX; // a_1 = floor(a_0 / 2^a_0) -> is always 0
//                     }
//                     reg[0] = u64::MAX; // could have been any value
//                 }
//                 let val = c(&reg, o);
//                 if val == u64::MAX {
//                     reg[(o - 4) as usize] = 0; // explore all values?
//                 }
//                 reg[0] = reg[0] * 1_u64 << c(&reg, o);
//             }
//         }
//         1 => {
//             if reg[1] != u64::MAX {
//                 if o == 5 {
//                     if reg[1] != 0 {
//                         return u64::MAX; // a_1 = a_1 ^ a_1 -> is always 0
//                     }
//                     reg[1] = u64::MAX; // could have been any value
//                 }
//                 let val = c(&reg, o);
//                 if val == u64::MAX {
//                     reg[(o - 4) as usize] = 0; // explore all values?
//                 }
//                 reg[1] = reg[1] ^ c(&reg, o);
//             }
//         }
//         5 => {
//             // printing the next value
//             let to_print = ins[p_i as usize];
//             let val = c(&reg, o);
//             if val == u64::MAX{
//                 reg[(o - 4) as usize] = to_print;
//             }
//             if val % 8 != to_print {
//                 return u64::MAX;
//             }
//             if p_i == 0 {
//                 return reg[0];
//             }
//         }
//         _ => {}
//     }
//     // correct configuration for printing, a previous step could be jumped, or one-step
//     // find places where we could jump to this instruction
//     let mut best = f_r(ins, reg, ptr - 2, p_i -1);
//     for i in (0..ins.len()).step_by(2) {
//         if ins[i] == 3 && ins[i + 1] == ptr {
//             best = best.min(f_r(ins, reg, i as u64, p_i-1));
//         }
//     }
//     best
// }

