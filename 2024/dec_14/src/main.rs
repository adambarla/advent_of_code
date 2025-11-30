use std::fs::File;
use std::io;
use std::io::Read;

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

fn m(rs: &Vec<R>, w: i32, h: i32, i: i32) -> Vec<R> {
    let mut tmp_rs = Vec::<R>::new();
    for r in rs.iter() {
        let x = (r.x + r.dx * i).rem_euclid(w);
        let y = (r.y + r.dy * i).rem_euclid(h);
        // *r = R { x, y, dx: r.dx, dy: r.dy };
        tmp_rs.push(R { x, y, dx: r.dx, dy: r.dy });
    }
    tmp_rs
}

fn main() {
    let mut s = String::new();
    let file = File::open("in").expect("Failed to open input file");
    io::BufReader::new(file).read_to_string(&mut s).expect("Failed to read input file");
    let rs: Vec<R> = s.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p: Vec<i32> = parts[0][2..].split(',').map(|s| s.parse().unwrap()).collect();
        let v: Vec<i32> = parts[1][2..].split(',').map(|s| s.parse().unwrap()).collect();
        R { x: p[0], y: p[1], dx: v[0], dy: v[1] }
    }).collect();
    // println!("{:?}", rs);

    let w = 101; // 11; // 101
    let h = 103; // 7; // 103

    let mut i = 84; // 0 // 7603 for the christmas tree to appear
    loop {
        print!("\x1B[2J");
        println!("i: {}", i);
        let tmp_rs = m(&rs, w, h, i);
        print(&tmp_rs, w, h);
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
    // part 1
    let sec = 100;
    let tmp_rs = m(&rs, w, h, sec);
    let sectors = get_sectors(&tmp_rs, w, h, 2);
    let mut prod = 1;
    for s in sectors.iter() {
        prod *= s;
    }
    println!("{}", prod);

}
