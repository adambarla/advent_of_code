use std::io;

fn val(v: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    if i >= v.len() || j >= v[i].len() {
        return u32::MAX;
    }
    v[i][j]
}

fn dfs(v: &Vec<Vec<u32>>, vis: &mut Vec<Vec<u32>>, i: usize, j: usize) -> (u32, u32) {
    if vis[i][j] == 1 {
        return (0, 0);
    }
    vis[i][j] = 1;
    let mut area = 1;
    let mut fence = 0;
    for (di, dj) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
        if v[i][j] != val(v, (i as i32 + di) as usize, (j as i32 + dj) as usize) {
            fence += 1;
        } else {
            let (a, f) = dfs(v, vis, (i as i32 + di) as usize, (j as i32 + dj) as usize);
            area += a;
            fence += f;
        }
    }
    (area, fence)
}

fn dfs2(v: &Vec<Vec<u32>>, vis: &mut Vec<Vec<u32>>, i: usize, j: usize) -> (u32, u32) {
    if vis[i][j] == 1 {
        return (0, 0);
    }
    // println!("{},{}: {:?}", i, j, v[i][j]);
    vis[i][j] = 1;
    let mut area = 1;
    let mut fence = 0;
    let mut op = Vec::<(usize, usize)>::new();
    for (di, dj) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
        op.push(((di + i as i32) as usize, (dj + j as i32) as usize));
    }
    let mut opd = Vec::<(usize, usize)>::new();
    for (di, dj) in vec![(1,1),(1,-1),(-1,-1),(-1,1)] {
        opd.push(((di + i as i32) as usize, (dj + j as i32) as usize));
    }
    if v[i][j] != val(v, op[0].0, op[0].1) && (
        v[i][j] != val(v, op[1].0, op[1].1) || v[i][j] == val(v, opd[0].0, opd[0].1))
    {
        fence += 1;
    }
    if v[i][j] != val(v, op[1].0, op[1].1) && (
        v[i][j] != val(v, op[2].0, op[2].1) || v[i][j] == val(v, opd[1].0, opd[1].1))
    {
        fence += 1;
    }
    if v[i][j] != val(v, op[2].0, op[2].1) && (
        v[i][j] != val(v, op[3].0, op[3].1) || v[i][j] == val(v, opd[2].0, opd[2].1))
    {
        fence += 1;
    }
    if v[i][j] != val(v, op[3].0, op[3].1) && (
        v[i][j] != val(v, op[0].0, op[0].1) || v[i][j] == val(v, opd[3].0, opd[3].1))
    {
        fence += 1;
    }
    for (i2, j2) in &op {
        if v[i][j] == val(v, *i2, *j2) {
            let (a, f) = dfs2(v, vis, *i2, *j2);
            area += a;
            fence += f;
        }
    }
    (area, fence)
}

fn main() {
    let mut v = Vec::<Vec<u32>>::new();
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("failed to read");
        let s = s.trim();
        if s.is_empty() {
            break;
        }
        let ch: Vec<u32> = s.chars().map(|c| c as u32).collect();
        v.push(ch);
    }
    // println!("{:?}", v);
    let mut vis = Vec::<Vec<u32>>::new();
    for i in 0..v.len() {
        let mut tmp = Vec::<u32>::new();
        for _ in 0..v[i].len() {
            tmp.push(0);
        }
        vis.push(tmp);
    }
    let mut price = 0;
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if vis[i][j] == 0 {
                let (a, f) = dfs2(&v, &mut vis, i, j);
                // println!("{:?}: {},{}", char::from_u32(v[i][j]), a, f);
                price += a * f;
            }
        }
    }
    println!("{}", price);
    // sum
}
