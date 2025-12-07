use std::collections::HashSet;
use std::fs;
use std::env;

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
    for l in file.split('\n') {
        if l.len() == 0 {
            break;
        }
        grid.push(l.chars().collect());
    }
    let h = grid.len();
    let w = grid[0].len();
    let start = grid[0].iter().position(|c| *c=='S').expect("start not found");
    println!("start {}", start);
    let mut beams = HashSet::<usize>::new();
    beams.insert(start);
    let mut prev_beams = HashSet::<usize>::new();
    let mut splits = 0;
    for r in 1..h {
        prev_beams = beams.clone();
        beams = HashSet::<usize>::new();
        for b in &prev_beams {
            if grid[r][*b] == '^' {
                beams.insert(b-1);
                beams.insert(b+1);
                splits += 1;
            }
            else {
                beams.insert(*b);
            }
        }
        // println!("{:?}", prev_beams);
        // println!("{:?}", beams);
        for i in 0..w {
            print!("{}",  if beams.contains(&i) {'|'} else {grid[r][i]} );
        }
        println!();
        // let new_splits = &beams.len() - &prev_beams.len();
        // println!("{}", new_splits);
        // splits += new_splits;
    }
    println!("part 1: {}", splits);
    // println!("{:?}", grid);
}
