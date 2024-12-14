use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(Debug)]
struct R{
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn get_sectors(rs: &Vec<R>, w: i32, h: i32, k: i32) -> Vec<i32> {
    let mut sectors = vec![0,0,0,0];
    for r in rs.iter() {
        let x = r.x;
        let y = r.y;
        if x < w / k && y < h / k {
            sectors[0] += 1;
        }
        else if x > w / k * (k - 1) && y < h / k {
            sectors[1] += 1;
        }
        else if x < w / k && y > h / k * (k - 1) {
            sectors[2] += 1;
        }
        else if x > w / k * (k - 1)&& y > h / k * (k - 1) {
            sectors[3] += 1;
        }
    }
    sectors
}

fn print(rs: &Vec<R>, w: i32, h: i32) {
    let mut grid = vec![vec![' '; w as usize]; h as usize];
    for r in rs.iter() {
        grid[r.y as usize][r.x as usize] = '#';
    }
    for row in grid.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

fn get_map(rs: &Vec<R> ) -> HashMap<(i32, i32, i32, i32), i32> {
    let mut m = HashMap::<(i32, i32, i32, i32), i32>::new();
    for r in rs.iter() {
        if let Some(val) = m.get_mut(&(r.x, r.y, r.dx, r.dy)) {
            *val += 1;
        }
        else{
            m.insert((r.x, r.y, r.dx, r.dy), 1);
        }
    }
    m
}

fn cmp_maps(m1: &HashMap<(i32, i32, i32, i32), i32>, m2: &HashMap<(i32, i32, i32, i32), i32>) -> bool {
    if m1.len() != m2.len() {
        return false;
    }
    for (k, v) in m1.iter() {
        if let Some(v2) = m2.get(k) {
            if v != v2 {
                return false;
            }
        }
        else {
            return false;
        }
    }
    true
}
// 3895
// 3998
fn main() {
    let mut s = String::new();
    let file = File::open("in").expect("Failed to open input file");
    io::BufReader::new(file).read_to_string(&mut s).expect("Failed to read input file");
    let mut rs: Vec<R> = s.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p: Vec<i32> = parts[0][2..].split(',').map(|s| s.parse().unwrap()).collect();
        let v: Vec<i32> = parts[1][2..].split(',').map(|s| s.parse().unwrap()).collect();
        R { x: p[0], y: p[1], dx: v[0], dy: v[1] }
    }).collect();
    // println!("{:?}", rs);

    let w = 101; // 11; // 101
    let h = 103; // 7; // 103

    let mut i = 84;
    // let mut m1 = get_map(&rs);
    loop {
        let mut tmp_rs = Vec::<R>::new();
        for r in rs.iter_mut() {
            let x = (r.x + r.dx * i).rem_euclid(w);
            let y = (r.y + r.dy * i).rem_euclid(h);
            // *r = R { x, y, dx: r.dx, dy: r.dy };
            tmp_rs.push(R { x, y, dx: r.dx, dy: r.dy });
        }
        // let mut m2 = get_map(&tmp_rs);
        // if cmp_maps(&m1, &m2){
        //     break;
        // }
        print!("\x1B[2J");
        println!("i: {}", i);
        print(&tmp_rs, w, h);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if input == "q" {
            break;
        }
        if input == "b" {
            i -= 103;
        }
        if input.is_empty() {
            i += 103;
        }
    }
    println!("{}", i);
    // let sec = i;
    // let mut prod = 1;
    // let sectors = get_sectors(&mut rs, w, h);
    // for s in sectors.iter() {
    //     prod *= s;
    // }
    // println!("{}", prod);

}
