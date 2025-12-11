use std::env;
use std::fs;
use std::collections::{HashSet, VecDeque, HashMap};
use priority_queue::PriorityQueue;
use z3;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize, SatResult};

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

fn press_button(state: &mut Vec<i32>, button: &Vec<usize>) {
    for &j in button {
        state[j] += 1;
    }
}

fn unpress_button(state: &mut Vec<i32>, button: &Vec<usize>) {
    for &j in button {
        state[j] -= 1;
    }
}

const PRIMES: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
const MOD: u128 = 1000000007;

fn hash_state(state: &Vec<i32>) -> u128 {
    let mut hash = 1u128;
    for i in 0..state.len() {
        hash = (hash * PRIMES[i].pow(state[i] as u32) as u128) % MOD;
    }
    return hash;
}

fn update_hash(hash: u128, button: &Vec<usize>) -> u128 {
    let mut new_hash = hash;
    for &j in button {
        new_hash = (new_hash * PRIMES[j] as u128) % MOD;
    }
    return new_hash;
}


fn solve_recursive(target_vec: &Vec<i32>, buttons: &Vec<Vec<usize>>, memo: &mut HashMap<Vec<i32>, u64>) -> u64 {
    if memo.contains_key(target_vec) {
        return *memo.get(target_vec).unwrap();
    }
    
    if target_vec.iter().all(|&x| x == 0) {
        return 0;
    }
    
    if target_vec.iter().any(|&x| x < 0) {
        return u64::MAX;
    }
    
    let mut best = u64::MAX;
    
    for btn in buttons {
        let mut remainder = target_vec.clone();
        for &j in btn {
            if j < remainder.len() {
                remainder[j] -= 1;
            }
        }
        
        let res = solve_recursive(&remainder, buttons, memo);
        if res != u64::MAX {
            best = best.min(res + 1);
        }
    }
    
    memo.insert(target_vec.clone(), best);
    return best;
}


fn z3_solve_machine(machine: &Machine) -> u64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    let mut button_vars = Vec::new();
    for i in 0..machine.buttons.len() {
        let var = Int::new_const(&ctx, format!("n_{}", i));
        opt.assert(&var.ge(&Int::from_i64(&ctx, 0)));
        button_vars.push(var);
    }
    for (i, target) in machine.jolts.iter().enumerate() {
        let mut sum = Int::from_i64(&ctx, 0);
        for (j, button) in machine.buttons.iter().enumerate() {
            if button.contains(&i) {
                sum = &sum + &button_vars[j];
            }
        }
        opt.assert(&sum._eq(&Int::from_i64(&ctx, *target as i64)));
    }
    let mut total_steps = Int::from_i64(&ctx, 0);
    for var in button_vars {
        total_steps = &total_steps + &var;
    }
    opt.minimize(&total_steps);

    if opt.check(&[]) == SatResult::Sat {
        let model = opt.get_model().unwrap();
        let presses = model.eval(&total_steps, true).unwrap();
        return presses.as_u64().unwrap_or(u64::MAX);
    } else {
        return u64::MAX;
    }


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
                // println!("Found goal state: {:?}, steps: {}", curr_state, curr_step);
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



    let mut total_steps_part_2: u64 = 0;

    // recursive approach
    // for m in &ms {
    //     let mut memo = HashMap::<Vec<i32>, u64>::new();
    //     let min_presses = solve_recursive(&m.jolts, &m.buttons, &mut memo);
    //     println!("Min presses to goal: {:?}", min_presses);
    //     total_steps_part_2 += min_presses;
    // }

    // bfs
    // for idx in 0..ms.len() {
    //     let m = &ms[idx];
    //     let goal_hash = hash_state(&m.jolts);
    //     let mut q = VecDeque::<(u128, u64)>::new();
    //     let mut steps: u64 = u64::MAX;
    //     q.push_back((1u128, 0));
    //     let mut visited = HashSet::<u128>::new();
    //     let mut max_step = 0;
    //     while !q.is_empty() {
    //         let (curr_hash, curr_step) = q.pop_front().unwrap();
    //         if curr_step > max_step {
    //             max_step = curr_step;
    //             println!("machine {}: max step: {:?}", idx + 1, max_step);
    //         }
    //         // println!("Current state: {:?}, steps: {}", curr_hash, curr_step);
    //         if curr_hash == goal_hash {
    //             println!("Found goal state: {:?}, steps: {}", curr_hash, curr_step);
    //             steps = curr_step;
    //             break;
    //         }
    //         for i in 0..m.buttons.len() {
    //             let new_hash = update_hash(curr_hash, &m.buttons[i]);
    //             if visited.contains(&new_hash) {
    //                 continue;
    //             }
    //             q.push_back((new_hash, curr_step + 1));
    //             visited.insert(new_hash);
    //         }
    //     }
    //     if steps == u64::MAX {
    //         panic!("No solution found for machine: {:?}", m);
    //     }
    //     total_steps_part_2 += steps;
    //     println!("machine {}: steps: {:?}", idx + 1, steps);
    // }

    // a* search
    // for idx in 0..ms.len() {
    //     let m = &ms[idx];
    //     let mut q = PriorityQueue::<(Vec<i32>, i32), i32>::new();
    //     let mut steps: i32 = i32::MAX;
    //     q.push((vec![0; m.jolts.len()], 0), 0);
    //     let mut visited = HashSet::<Vec<i32>>::new();
    //     let mut max_step = 0;
    //     while !q.is_empty() {
    //         let ((curr_state, curr_step), cost) = q.pop().unwrap();
    //         // println!("Current state: {:?}, steps: {}, cost: {}", curr_state, curr_step, cost);
    //         if curr_step > max_step {
    //             max_step = curr_step;
    //             println!("machine {}: max step: {:?}, cost: {:?}", idx + 1, max_step, cost);
    //         }
    //         // println!("Current state: {:?}, steps: {}", curr_hash, curr_step);
    //         if curr_state == m.jolts {
    //             // println!("Found goal state: {:?}, steps: {}", curr_state, curr_step);
    //             steps = curr_step;
    //             break;
    //         }
    //         'outer: for i in 0..m.buttons.len() {
    //             let mut new_state = curr_state.clone();
    //             for j in &m.buttons[i] {
    //                 new_state[*j] += 1;
    //             }
    //             if visited.contains(&new_state) {
    //                 continue;
    //             }
    //             for j in 0..m.jolts.len() {
    //                 if new_state[j] > m.jolts[j] {
    //                     continue 'outer;
    //                 }
    //             }
    //             let mut distance = 0;
    //             for j in 0..m.jolts.len() {
    //                 let diff = (new_state[j] - m.jolts[j]).abs();
    //                 distance = std::cmp::max(distance, diff);
    //             }
    //             let priority = -(curr_step + distance + 1);
    //             visited.insert(new_state.clone());
    //             q.push((new_state, curr_step + 1), priority);
    //         }
    //     }
    //     if steps == i32::MAX {
    //         panic!("No solution found for machine: {:?}", m);
    //     }
    //     total_steps_part_2 += steps as u64;
    //     println!("machine {}: steps: {:?}", idx + 1, steps);
    // }

    // z3 approach
    for (idx, m) in ms.iter().enumerate() {
        let min_presses = z3_solve_machine(m);
        // println!("machine {}: min presses: {:?}", idx + 1, min_presses);
        total_steps_part_2 += min_presses;
    }

    println!("part 2: {}", total_steps_part_2);
}
