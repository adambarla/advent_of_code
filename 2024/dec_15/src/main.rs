use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead};
use std::{env, io};

fn r(file: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let f = File::open(file).expect("Failed to open input file");
    let mut reader = io::BufReader::new(f);
    let mut gr = Vec::<Vec<char>>::new();
    loop {
        let mut line = String::new();
        // std::io::stdin().read_line(&mut line).unwrap();
        reader.read_line(&mut line).expect("Failed to read line");
        line = line.trim().to_string();
        if line.is_empty() {
            break;
        }
        let row: Vec<char> = line.chars().collect();
        gr.push(row);
    }
    let mut mv = Vec::<char>::new();
    loop {
        let mut line = String::new();
        // std::io::stdin().read_line(&mut line).unwrap();
        reader.read_line(&mut line).expect("Failed to read line");
        line = line.trim().to_string();
        if line.is_empty() {
            break;
        }
        mv.extend(line.chars());
    }
    (gr, mv)
}

fn f_r(gr: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..gr.len() {
        for j in 0..gr[i].len() {
            if gr[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("No start point");
}

// part 1
// fn m(gr: &mut Vec<Vec<char>>, r: &mut (i32, i32), d: (i32, i32)) {
//     let (x, y) = *r;
//     let (dx, dy) = d;
//     if gr[(x + dx) as usize][(y + dy) as usize] == '.' {
//         gr[(x + dx) as usize][(y + dy) as usize] = '@';
//         *r = (x + dx, y + dy);
//         gr[x as usize][y as usize] = '.';
//         return;
//     }
//     let mut i = 1;
//     while gr[(x + i * dx) as usize][(y + i * dy) as usize] == 'O' {
//         i += 1
//     }
//     if gr[(x + i * dx) as usize][(y + i * dy) as usize] == '.' {
//         gr[(x + i * dx) as usize][(y + i * dy) as usize] = 'O';
//         gr[(x + dx) as usize][(y + dy) as usize] = '@';
//         *r = (x + dx, y + dy);
//         gr[x as usize][y as usize] = '.';
//         return;
//     }
// }
//
// fn f_b(gr: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
//     let mut b = Vec::<(i32, i32)>::new();
//     for i in 0..gr.len() {
//         for j in 0..gr[i].len() {
//             if gr[i][j] == 'O' {
//                 b.push((i as i32, j as i32));
//             }
//         }
//     }
//     b
// }

fn w(gr: &mut Vec<Vec<char>>) {
    let mut n_gr = Vec::with_capacity(2 * gr.len());
    for l in gr.iter() {
        let mut n_l = Vec::with_capacity(2 * l.len());
        for &c in l {
            match c {
                'O' => {
                    n_l.push('[');
                    n_l.push(']');
                }
                '@' => {
                    n_l.push('@');
                    n_l.push('.');
                }
                '.' => {
                    n_l.push('.');
                    n_l.push('.');
                }
                '#' => {
                    n_l.push('#');
                    n_l.push('#');
                }
                _ => panic!("Invalid character"),
            }
        }
        n_gr.push(n_l);
    }
    *gr = n_gr;
}

fn m_lr(gr: &mut Vec<Vec<char>>, r: &mut (i32, i32), dy: i32) {
    let mut i = 1;
    while gr[r.0 as usize][(r.1 + i * dy) as usize] == '['
        || gr[r.0 as usize][(r.1 + i * dy) as usize] == ']'
    {
        i += 1;
    }
    if gr[r.0 as usize][(r.1 + i * dy) as usize] == '#' {
        return;
    }
    for j in (1..i + 1).rev() {
        gr[r.0 as usize][(r.1 + j * dy) as usize] = gr[r.0 as usize][(r.1 + (j - 1) * dy) as usize];
    }
    gr[r.0 as usize][r.1 as usize] = '.';
    *r = (r.0, r.1 + dy);
}

fn m_ud(gr: &mut Vec<Vec<char>>, r: &mut (i32, i32), dx: i32) {
    if gr[(r.0 + dx) as usize][r.1 as usize] == '#' {
        return;
    }
    if gr[(r.0 + dx) as usize][r.1 as usize] == '.' {
        gr[(r.0 + dx) as usize][r.1 as usize] = '@';
        gr[r.0 as usize][r.1 as usize] = '.';
        *r = (r.0 + dx, r.1);
        return;
    }

    let mut q1 = VecDeque::<(i32, i32)>::new();
    let mut q2 = VecDeque::<(i32, i32)>::new();
    let mut s = HashSet::<(i32, i32)>::new();
    if gr[(r.0 + dx) as usize][r.1 as usize] == '[' {
        q1.push_back((r.0 + dx, r.1));
        s.insert((r.0 + dx, r.1));
    } else if gr[(r.0 + dx) as usize][r.1 as usize] == ']' {
        q1.push_back((r.0 + dx, r.1 - 1));
        s.insert((r.0 + dx, r.1 - 1));
    } else {
        panic!("Invalid character");
    }

    while let Some((x, y)) = q1.pop_front() {
        s.remove(&(x, y));
        if gr[(x + dx) as usize][y as usize] == '#' || gr[(x + dx) as usize][y as usize + 1] == '#'
        {
            return;
        }
        if gr[x as usize][y as usize] == '[' {
            if gr[(x + dx) as usize][y as usize] == '[' {
                if !s.contains(&(x + dx, y)) {
                    q1.push_back((x + dx, y));
                    s.insert((x + dx, y));
                }
            } else {
                if gr[(x + dx) as usize][y as usize] == ']' {
                    if !s.contains(&(x + dx, y - 1)) {
                        q1.push_back((x + dx, y - 1));
                        s.insert((x + dx, y - 1));
                    }
                }
                if gr[(x + dx) as usize][y as usize + 1] == '[' {
                    if !s.contains(&(x + dx, y + 1)) {
                        q1.push_back((x + dx, y + 1));
                        s.insert((x + dx, y + 1));
                    }
                }
            }
        }
        q2.push_back((x, y));
    }
    while let Some((x, y)) = q2.pop_back() {
        gr[(x + dx) as usize][y as usize] = gr[x as usize][y as usize];
        gr[(x + dx) as usize][y as usize + 1] = gr[x as usize][y as usize + 1];
        gr[x as usize][y as usize] = '.';
        gr[x as usize][y as usize + 1] = '.';
    }
    gr[(r.0 + dx) as usize][r.1 as usize] = '@';
    gr[r.0 as usize][r.1 as usize] = '.';
    *r = (r.0 + dx, r.1);
}

fn f_b2(gr: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut b = Vec::<(i32, i32)>::new();
    for i in 0..gr.len() {
        for j in 0..gr[i].len() {
            if gr[i][j] == '[' {
                b.push((i as i32, j as i32));
            }
        }
    }
    b
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <input file>", args[0]);
    }
    let (mut gr, mvs) = r(&args[1]);
    w(&mut gr);
    let mut r = f_r(&gr);
    let map = HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);
    for mv in mvs.iter() {
        let (dx, dy) = map.get(&mv).unwrap();
        if *mv == '<' || *mv == '>' {
            m_lr(&mut gr, &mut r, *dy);
        } else {
            m_ud(&mut gr, &mut r, *dx);
        }
    }
    let b = f_b2(&gr);
    let mut sum = 0;
    for (i, j) in b {
        sum += 100 * i + j;
    }
    println!("{}", sum);
}
