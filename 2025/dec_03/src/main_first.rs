use std::fs;
use std::env;

fn main() {
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
    let mut total: i64 = 0;
    for line in file.split('\n') {
        let res: Result<Vec<i32>, _> = line.chars()
            .collect::<Vec<char>>().iter()
            .map(|c| c.to_string().parse::<i32>()).collect();
        let digits = match res {
            Ok(d) => d,
            Err(_) => break
        };
        if digits.len() < 2 {
            println!("finist because len < 2 {:?}", digits);
            break;
        }
        let mut best_1 = 0;
        for i in 1..digits.len()-1 {
            if digits[i] > digits[best_1] {
                best_1 = i;
            }
        }
        let mut best_2 = best_1 + 1;
        for i in best_1+1..digits.len() {
            if digits[i] > digits[best_2] {
                best_2 = i;
            }
        }
        let num = digits[best_1]*10 + digits[best_2];
        total += num as i64;
        println!("{}", num);
    }
    println!("{}",total);

}
