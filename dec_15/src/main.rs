use std::collections::HashMap;

fn r_gr() -> Vec<Vec<char>> {
    let mut gr = Vec::<Vec<char>>::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.is_empty() {
            break;
        }
        let row: Vec<char> = line.chars().collect();
        gr.push(row);
    }
    gr
}

fn r_mv() -> Vec<char> {
    let mut mv = Vec::<char>::new();
    let mut line = String::new();
    loop{
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.is_empty() {
            break;
        }
        mv.extend(line.chars());
    }
    mv
}

fn f_r(gr: &Vec<Vec<char>>) -> (i32,i32) {
    for i in 0..gr.len() {
        for j in 0..gr[i].len() {
            if gr[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("No start point");
}

fn m(gr: &mut Vec<Vec<char>>, r: &mut (i32,i32), d: (i32, i32)) {
    let (x, y) = *r;
    let (dx, dy) = d;
    if gr[(x + dx) as usize][(y + dy) as usize] == '.' {
        gr[(x + dx) as usize][(y + dy) as usize] = '@';
        *r = (x + dx, y + dy);
        gr[x as usize][y as usize] = '.';
        return;
    }
    let mut i = 1;
    while gr[(x + i*dx) as usize][(y + i*dy) as usize] == 'O' {
        i += 1
    }
    if gr[(x + i*dx) as usize][(y + i*dy) as usize] == '.' {
        gr[(x + i*dx) as usize][(y + i*dy) as usize] = 'O';
        gr[(x + dx) as usize][(y + dy) as usize] = '@';
        *r = (x + dx, y + dy);
        gr[x as usize][y as usize] = '.';
        return;
    }
}

fn f_b(gr: &Vec<Vec<char>>) -> Vec<(i32,i32)> {
    let mut b = Vec::<(i32,i32)>::new();
    for i in 0..gr.len() {
        for j in 0..gr[i].len() {
            if gr[i][j] == 'O' {
                b.push((i as i32, j as i32));
            }
        }
    }
    b
}

fn main() {
    let mut gr = r_gr();
    let mut r = f_r(&gr);
    let mvs = r_mv();
    let map = HashMap::from([
        ('^', (-1, 0)),
        ('v', (1, 0)),
        ('<', (0, -1)),
        ('>', (0, 1)),
    ]);
    for mv in mvs {
        let (dx, dy) = map.get(&mv).unwrap();
        m(&mut gr, &mut r, (*dx, *dy));
    }
    // for row in gr {
    //     println!("{}", row.iter().collect::<String>());
    // }
    let b = f_b(&gr);
    let mut sum = 0;
    for (i, j) in b {
        sum += 100*i + j;
    }
    println!("{}", sum);
}

