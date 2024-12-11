use std::collections::HashMap;
use std::io;

fn hb(a: usize, n: usize, m: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return 1;
    }
    if let Some(&val) = m.get(&(a, n)) {
        return val;
    }
    if a == 0 {
        let res = hb(1, n - 1, m);
        m.insert((a, n), res);
        return res;
    }
    let str = a.to_string();
    if str.len() % 2 == 0 {
        let half = str.len() / 2;
        let (b, c) = str.split_at(half);
        let b: usize = b.parse().expect("failed to parse");
        let c: usize = c.parse().expect("failed to parse");
        let res = hb(b, n - 1, m) + hb(c, n - 1, m);
        m.insert((a, n), res);
        return res;
    }
    let res = hb(a * 2024, n - 1, m);
    m.insert((a, n), res);
    return res;
}

fn main() {
    let n = 75;
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read");
    let s = s.trim();
    let mut v: Vec<usize> = s
        .split(' ')
        .map(|s| s.parse().expect("failed to parse"))
        .collect();
    println!("{:?}", v);
    let mut map = HashMap::<(usize, usize), usize>::new();
    let mut count = 0;
    for i in 0..v.len() {
        count += hb(v[i], n, &mut map);
    }
    println!("{}", count);
}
