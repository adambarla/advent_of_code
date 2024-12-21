use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

static N: i32 = 3;

#[derive(Eq, Ord)]
struct P { // State of a robot
    pointed_at: char,
    last_command: char,
    distance: i32,
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

fn get_ch2ps(kn: &Vec<Vec<char>>, kd: &Vec<Vec<char>>) -> HashMap<(char, bool), (i32, i32)> {
    let mut m = HashMap::<(char, bool), (i32, i32)>::new();
    for i in 0..kn.len() {
        for j in 0..kn[i].len() {
            m.insert((kn[i][j], true), (i as i32, j as i32));
        }
    }
    for i in 0..kd.len() {
        for j in 0..kd[i].len() {
            m.insert((kd[i][j], false), (i as i32, j as i32));
        }
    }
    m
}

fn dir2vec(ch: char) -> (i32, i32) {
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

fn is_valid(v: &Vec<Vec<char>>, p: (i32, i32)) -> bool {
    return p.0 >= 0
        && p.0 < v.len() as i32
        && p.1 >= 0
        && p.1 < v[p.0 as usize].len() as i32
        && v[p.0 as usize][p.1 as usize] != ' '
}

fn dist(
    s_ch: char,
    e_ch: char,
    k: i32,
    kn: &Vec<Vec<char>>,
    kd: &Vec<Vec<char>>,
    ch2ps: &HashMap<(char, bool), (i32, i32)>,
    memo: &mut HashMap<(char, char, i32), i32>
) -> i32 {
    if k == N {
        return 1;
    }

    // println!();
    if memo.contains_key(&(s_ch,e_ch,k)) {
        return memo[&(s_ch,e_ch,k)];
    }

    let first = k==0;
    let pad = if first { kn } else { kd };
    let mut que = BinaryHeap::<P>::new();
    let mut vis = HashMap::<char,(i32,char)>::new();
    que.push(P {
        pointed_at: s_ch,
        last_command: 'A',
        distance: 0,
    });
    vis.insert(s_ch,(0,' '));

    while let Some(a) = que.pop() {
        // println!("ch({}): {} ch({}): {} d={} \t(s: {} e: {})",k, a.ch, k+1,a.prev, a.d, s_ch, e_ch);
        if a.pointed_at == e_ch {
            let d = a.distance + dist(a.last_command, 'A', k + 1, kn, kd, ch2ps, memo);
            memo.insert((s_ch,e_ch,k), d );
            return d;
        }
        for new_command in "<^>v".chars() {
            let (di, dj) = dir2vec(new_command);
            let p = ch2ps[&(a.pointed_at,first)];
            let new_p = (p.0 + di, p.1 + dj);
            if !is_valid(&pad, new_p) {
                continue;
            }
            let new_ch = pad[new_p.0 as usize][new_p.1 as usize];
            let d = a.distance + dist(a.last_command, new_command, k + 1, kn, kd, ch2ps, memo);
            if vis.contains_key(&new_ch) && vis[&new_ch].0 >= d {
                continue;
            }
            que.push(P {
                pointed_at: new_ch,
                last_command: new_command,
                distance: d,
            });
            vis.insert(new_ch, (d,a.pointed_at));
        }
    }
    panic!("no path");
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("usage: program <file>")
    }
    let f = File::open(&args[1]).expect("couldn't read file");
    let mut r = BufReader::new(&f);
    let mut codes = Vec::<(i32, Vec<char>)>::new();
    loop {
        let mut s = String::new();
        r.read_line(&mut s).expect("couldn't read line");
        let s = s.trim().to_string();
        if s.is_empty() {
            break;
        }
        let v = s.chars().collect();
        let n = s[..s.len() - 1].parse::<i32>().unwrap();
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
    let mut memo = HashMap::<(char, char, i32), i32>::new();

    let mut sum = 0;
    for (n, code) in codes {
        let mut part_sum = 0;
        let mut prev = 'A';
        for ch in code {
            let d = dist(prev, ch, 0, &kn, &kd, &ch2ps, &mut memo);
            part_sum += d;
            prev = ch;
        }
        println!("{} {}",n, part_sum);
        sum += n *  part_sum;
    }
    println!("{}", sum);
}
