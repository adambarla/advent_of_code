use std::fs;
use std::env;

const N : i32 = -1; // -1 for no limit ** (stops after no change), 1 for *

fn get_file() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide input file path as an argument!")
    }
    let file_path = &args[1];
    let file = fs::read_to_string(file_path);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            panic!("Error when reading file: {}", e)
        }
    };
    return file;
}

fn main() {
    let file = get_file();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut h : i32 = 0;
    let mut w : i32 = 0;
    for line in file.split('\n') {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() == 0 {
            break;
        }
        if w == 0 {
            w = chars.len() as i32;
        }
        else if w != chars.len() as i32 {
            panic!("input isn't a grid, line lenght {} != {}", w ,chars.len());
        }
        grid.push(chars);
        h += 1;
    }
    let mut count = 0;
    let mut loops = 0;
    loop {
        if N != -1 && loops >= N {
            break;
        }
        let mut remove = vec![vec![0; w as usize]; h as usize];
        for i in 0..h {
            for j in 0..w {
                print!("{}",grid[i as usize][j as usize]);
            }
            println!();
        }
        for i in 0..h {
            'row : for j in 0..w {
                if grid[i as usize][j as usize] != '@' {
                    continue;
                }
                let mut surronded = 0;
                for i_d in i-1..i+2 {
                    for j_d in j-1..j+2 {
                        if (i_d == i && j_d == j)
                        || i_d < 0 || i_d >= h || j_d < 0 || j_d >= w {
                            continue;
                        }
                        if grid[i_d as usize][j_d as usize] == '@' {
                            surronded += 1;
                        }
                        if surronded >= 4 {
                            continue 'row;
                        }
                    }
                }
                remove[i as usize][j as usize] = 1;
                count += 1;
            }
        }
        println!("{}", count);
        loops += 1;

        // stop if no change
        let mut changed = 0;
        for i in 0..h {
            for j in 0..w {
                if remove[i as usize][j as usize] == 1 {
                    grid[i as usize][j as usize] = '.';
                    changed += 1;
                }
            }
        }
        if changed == 0 {
            break;
        }
    }
}
