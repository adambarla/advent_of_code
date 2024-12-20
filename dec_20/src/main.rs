use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn bfs(m: &Vec<Vec<u8>>, s: (i32, i32), e:(i32, i32)) -> Option<i32> {
    let mut v = HashMap::<(i32, i32), i32>::new();
    let mut q = VecDeque::<(i32, i32)>::new();

    q.push_back(s);
    v.insert(s,0);

    while let Some(a) = q.pop_front() {
        for (di,dj) in [(0,1), (1,0), (0,-1), (-1,0)] {
            let b = (a.0 + di, a.1 + dj);
            if v.contains_key(&b)
                ||  b.0 < 0 || b.0 >= m.len() as i32 || b.1 < 0 || b.1 >= m[0].len() as i32
                || m[b.0 as usize][b.1 as usize] as char == '#' {
                continue;
            }
            q.push_back(b);
            v.insert(b,v[&a] + 1);
        }
    }
    let res: Option<i32> = v.get(&e).cloned();
    res
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
    let og = bfs(&m,s,e).unwrap();
    let mut c = 0;
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] as char != '#'{
                continue
            }
            m[i][j] = ' ' as u8;
            let new = bfs(&m, s, e).unwrap();
            m[i][j] = '#' as u8;
            if og - new >= 100 {
                c += 1;
                println!("({},{}) -> {}",i,j,og - new);
            }
        }
    }
    println!("{}",c);

}
