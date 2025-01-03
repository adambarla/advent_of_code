use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

static mut N: i64 = 26; // number of robots

#[derive(Eq, Ord)]
struct P {
    // State of a robot
    head: char,
    last_command: char,
    distance: i64,
}

impl PartialEq<Self> for P {
    fn eq(&self, other: &Self) -> bool {
        other.distance == self.distance
    }
}

impl PartialOrd for P {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

fn get_ch2ps(kn: &Vec<Vec<char>>, kd: &Vec<Vec<char>>) -> HashMap<(char, bool), (i64, i64)> {
    let mut m = HashMap::<(char, bool), (i64, i64)>::new();
    for i in 0..kn.len() {
        for j in 0..kn[i].len() {
            m.insert((kn[i][j], true), (i as i64, j as i64));
        }
    }
    for i in 0..kd.len() {
        for j in 0..kd[i].len() {
            m.insert((kd[i][j], false), (i as i64, j as i64));
        }
    }
    m
}

fn dir2vec(ch: char) -> (i64, i64) {
    match ch {
        '<' => (0, -1),
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        _ => {
            panic!("direction not valid")
        }
    }
}

fn is_valid(v: &Vec<Vec<char>>, p: (i64, i64)) -> bool {
    return p.0 >= 0
        && p.0 < v.len() as i64
        && p.1 >= 0
        && p.1 < v[p.0 as usize].len() as i64
        && v[p.0 as usize][p.1 as usize] != ' ';
}

fn dist(
    s_ch: char,
    e_ch: char,
    k: i64,
    kn: &Vec<Vec<char>>,
    kd: &Vec<Vec<char>>,
    ch2ps: &HashMap<(char, bool), (i64, i64)>,
    memo: &mut HashMap<(char, char, i64), i64>,
    n: i64,
) -> i64 {
    if k == n {
        return 1;
    }

    // println!();
    if memo.contains_key(&(s_ch, e_ch, k)) {
        return memo[&(s_ch, e_ch, k)];
    }

    let first = k == 0;
    let pad = if first { kn } else { kd };
    let mut que = BinaryHeap::<P>::new();
    let mut vis = HashMap::<(char, char), i64>::new();
    que.push(P {
        head: s_ch,
        last_command: 'A',
        distance: 0,
    });
    vis.insert((s_ch, 'A'), 0);

    let mut best = i64::MAX;
    while let Some(a) = que.pop() {
        if a.head == e_ch {
            let d = a.distance + dist(a.last_command, 'A', k + 1, kn, kd, ch2ps, memo, n);
            if best > d {
                best = d;
            }
            continue;
        }
        for new_command in "<^>v".chars() {
            let (di, dj) = dir2vec(new_command);
            let p = ch2ps[&(a.head, first)];
            let new_p = (p.0 + di, p.1 + dj);
            if !is_valid(&pad, new_p) {
                continue;
            }
            let new_head = pad[new_p.0 as usize][new_p.1 as usize];
            let d = a.distance + dist(a.last_command, new_command, k + 1, kn, kd, ch2ps, memo, n);
            if let Some(&old_cost) = vis.get(&(new_head, new_command)) {
                if old_cost <= d {
                    continue;
                }
            }
            que.push(P {
                head: new_head,
                last_command: new_command,
                distance: d,
            });
            vis.insert((new_head, new_command), d);
        }
    }
    if best == i64::MAX {
        panic!("no path");
    }
    memo.insert((s_ch, e_ch, k), best);
    best
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        panic!("usage: program <file> <N>")
    }
    let n = args[2].parse::<i64>().expect("usage: program <file> [N]"); // n.o. robots, *3, **26
    let f = File::open(&args[1]).expect("couldn't read file");
    let mut r = BufReader::new(&f);
    let mut codes = Vec::<(i64, Vec<char>)>::new();
    loop {
        let mut s = String::new();
        r.read_line(&mut s).expect("couldn't read line");
        let s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        let v = s.chars().collect();
        let n = s[..s.len() - 1].parse::<i64>().unwrap();
        codes.push((n, v));
    }
    let kn: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ];
    let kd: Vec<Vec<char>> = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];
    let ch2ps = get_ch2ps(&kn, &kd);
    let mut memo = HashMap::<(char, char, i64), i64>::new();

    let mut sum = 0;
    for (a, code) in codes {
        let mut part_sum = 0;
        let mut prev = 'A';
        for ch in code {
            let d = dist(prev, ch, 0, &kn, &kd, &ch2ps, &mut memo, n);
            part_sum += d;
            prev = ch;
        }
        println!("{} {}", a, part_sum);
        sum += a * part_sum;
    }
    println!("{}", sum);
}
