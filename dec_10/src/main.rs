use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::io;

fn is_valid(i:i32,j:i32,n:u32) -> bool {
    if i >= 0 && i < n as i32 && j >= 0 && j < n as i32 {
        return true;
    }
    false
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    p : (i32,i32),
    d : i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn count_paths(square: &Vec<Vec<u32>>, start: (i32,i32)) -> u32 {
    if square[start.0 as usize][start.1 as usize] != 0 {
        return 0;
    }
    let n = square.len() as u32;
    let mut q = BinaryHeap::<Point>::new();
    let mut visited = Vec::<Vec<u32>>::new();
    for _ in 0..n {
        let mut row = Vec::<u32>::new();
        for _ in 0..n {
            row.push(0);
        }
        visited.push(row);
    }
    q.push(Point{p: start,d:0});
    visited[start.0 as usize][start.1 as usize] = 1;
    let mut count = 0;
    while let Some(a) = q.pop() {
        let (i,j) = a.p;
        let d = a.d;
        if square[i as usize][j as usize] == 9 {
            count += visited[i as usize][j as usize];
        }
        for (di,dj) in vec![(0,1),(1,0),(0,-1),(-1,0)] {
            let ni = i + di;
            let nj = j + dj;
            if is_valid(ni,nj,n)
            && square[ni as usize][nj as usize] == (d + 1) as u32 {
                if visited[ni as usize][nj as usize] == 0 {
                    q.push(Point{p:(ni,nj),d:d+1});
                }
                visited[ni as usize][nj as usize] += visited[i as usize][j as usize];

            }
        }
    }
    count
}

fn main() {
    let mut square = Vec::<Vec<u32>>::new();
    loop{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("failed to read");
        let s = s.trim();
        if s.is_empty() {
            break;
        }
        let ch: Vec<u32> = s
            .chars()
            .map(|s| s.to_digit(10).unwrap_or(u32::MAX))
            .collect();
        square.push(ch);
    }
    let mut count = 0;
    for i in 0..square.len() {
        for j in 0..square.len() {
            if square[i][j] != 0 {
                // print!(".");
                continue;
            }
            let tmp = count_paths(&square,(i as i32,j as i32));
            // print!("{}",tmp);
            count += tmp;
        }
        // println!();
    }
    println!("{}",count);

}
