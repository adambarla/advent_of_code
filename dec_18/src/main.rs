use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

fn bfs(g: &Vec<Vec<i64>>, n: usize) -> i64 {
    let mut q = Vec::<(usize, usize)>::new();
    let mut v = vec![vec![0; g[0].len()]; g.len()];
    q.push((0,0));
    v[0][0] = 1;
    while !q.is_empty() {
        let (x, y) = q.remove(0);
        if (x, y) == (n,n) {
            return v[x][y] - 1;
        }
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0 && nx <= n as i64 && ny >= 0 && ny <= n as i64
                && g[nx as usize][ny as usize] == 0 && v[nx as usize][ny as usize] == 0 {
                q.push((nx as usize, ny as usize));
                v[nx as usize][ny as usize] = v[x][y] + 1;
            }
        }
    }
    -1
}

fn main() {
    let n = 70; // 70
    // let k = 1024;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage \"cargo run <file>\"")
    }
    let f = File::open(&args[1]).expect("couldn't open file");
    let mut r = BufReader::new(f);
    let mut g = vec![vec![0; n+1]; n+1];
    let mut s = String::new();
    let mut i = 0;
    loop {
        s.clear();
        r.read_line(&mut s).expect("couldn't read line");
        s = s.trim().to_string();
        if s.is_empty(){
            break;
        }
        let pair = s
            .split(",")
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        g[pair[0] as usize][pair[1] as usize] = 1;
        i += 1;
        let a = bfs(&g,n);
        if a == -1 {
            println!("{:?}",pair);
            break;
        }
        println!("{}", a);
    }
    // for n in v {
    //     print!("{},",n);
    // }
    // println!();
}
