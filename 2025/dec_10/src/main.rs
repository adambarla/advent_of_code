use std::env;
use std::fs;
use std::collections::{HashSet, VecDeque};

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

#[derive(Debug)]
struct Machine {
    goals: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    jolts: Vec<i32>,
}


fn main() {
    let file = get_file();
    let lines = file.lines().collect::<Vec<&str>>();

    let mut ms = Vec::<Machine>::new();
    for line in lines {
        println!("{:?}", line);
        // [goals] (i0,i1,i2,...) (j0,j1,j2,...) ... buttons
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        // goals are . or # wrapped in [], lets remove first and last character
        // then get characters and convert to bools (. off and # on)
        let goals = parts[0][1..parts[0].len()-1]
            .chars().map(|c| c == '#').collect::<Vec<bool>>();

        // buttons are from idx 1 to second last, wrapped in ()
        let mut buttons = Vec::<Vec<usize>>::new();
        for i in 1..parts.len()-1 {
            let button = parts[i][1..parts[i].len()-1]
                .split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            buttons.push(button);
        }

        // jolts are from last idx, wrapped in {}
        let jolts = parts[parts.len()-1][1..parts[parts.len()-1].len()-1]
            .split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        ms.push(Machine { goals, buttons, jolts });
    }
    for m in &ms {
        println!("{:?}", m);
    }
    // test machine 0
    let mut total_steps: u64 = 0;
    for m in &ms {
        // each machine starts from false state, lets build a graph of all possible states using buttons
        let mut q = VecDeque::<(Vec<bool>, u64)>::new();
        let mut steps: u64 = u64::MAX;
        q.push_back((vec![false; m.goals.len()], 0));
        let mut visited = HashSet::<Vec<bool>>::new();
        while !q.is_empty() {
            let (curr_state, curr_step) = q.pop_front().unwrap();
            if curr_state == m.goals {
                println!("Found goal state: {:?}, steps: {}", curr_state, curr_step);
                steps = curr_step;
                break;
            }
            for i in 0..m.buttons.len() {
                let mut new_state = curr_state.clone();
                for j in &m.buttons[i] {
                    new_state[*j] = !new_state[*j];
                }
                if visited.contains(&new_state) {
                    continue;
                }
                q.push_back((new_state.clone(), curr_step + 1));
                visited.insert(new_state.clone());
            }
        }
        if steps == u64::MAX {
            panic!("No solution found for machine: {:?}", m);
        }
        total_steps += steps;
    }
    println!("part 1: {}", total_steps);
}
