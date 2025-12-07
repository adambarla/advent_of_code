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
    let mut beams: Vec<i64> = vec![0;w];
    beams[start] = 1;
    let mut splits = 0;
    for r in 1..h {
        for b in 0..w {
            if beams[b] == 0 {
                continue;
            }
            if grid[r][b] == '^' {
                if b >= 1 {
                    beams[b-1] += beams[b];
                }
                if b+1 < w {
                    beams[b+1] += beams[b];
                }
                beams[b] = 0;
                splits += 1;
            }
        }
        // println!("{:?}", prev_beams);
        // println!("{:?}", beams);
        for i in 0..w {
            print!("{}",  if beams[i] != 0 {'|'} else {grid[r][i]} );
        }
        print!(" {:?}", beams);
        println!();
    }
    println!("part 1: {}", splits);
    let mut timelines = 0;
    for i in 0..w {
        timelines += beams[i];
    }
    println!("part 2: {}", timelines);
    // println!("{:?}", grid);
}
