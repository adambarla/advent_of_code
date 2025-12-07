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
    // read numners
    let lines = file.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
    let mut l = 0;
    let mut problems : Vec<Vec<i64>> = Vec::<Vec<i64>>::new();
    loop {
        let line = &lines[l]; 
        let fields = line.split_ascii_whitespace().collect::<Vec<&str>>();
        // try parsing as i64
        let res : Result<Vec<i64>,_> = fields.iter()
            .map(|s| s.parse::<i64>()).collect();
        let nums = match res {
            Ok(ns) => ns,
            Err(_) => break, // end of numbers
        };
        problems.push(nums);
        if l > 0 && problems[0].len() != problems[l].len() {
            panic!("wrong input, inconsistend number of problems")
        }
        l += 1;
    }
    // println!("{:?}", problems);
    // read operations
    let line: String = lines[l].chars().filter(|c| !c.is_whitespace()).collect();
    if l > 0 && problems[0].len() != line.len() {
        panic!("wrong input, inconsistend number of problems and oparations")
    }
    let mut glob_sum = 0;
    for p in 0..problems[0].len() {
        let mut res = problems[0][p];
        for n in 1..problems.len() {
            let c = line.chars().nth(p).expect("no operation");
            match c {
                '*' => res *= problems[n][p as usize],
                '+' => res += problems[n][p as usize],
                _ => panic!("invalid operation {}", c)
            }
        }
        glob_sum += res;
    }
    println!("part 1: {}", glob_sum);

    // for part 2 construct the numbers in problems
    let mut lengths = vec![0; problems[0].len()];
    for p in 0..lengths.len() {
        for n in 0..problems.len() {
            let l = problems[n][p].to_string().len();
            if l > lengths[p]{
                lengths[p] = l;
            }
        }
    }
    let mut problem_strings = Vec::<Vec<String>>::new();
    for n in 0..problems.len(){
        let line = &lines[n];
        let mut strings = Vec::<String>::new();
        let mut offset = 0;
        for p in 0..lengths.len() {
            let s = &line[offset..offset+lengths[p]];
            strings.push(s.to_string());
            offset += lengths[p] + 1;
        }
        problem_strings.push(strings);
    }
    // println!("{:?}", problem_strings);
    glob_sum = 0;
    for p in 0..problems[0].len() {
        let mut numbers = Vec::<i64>::new();
        for pos in 0..problem_strings[0][p].len() {
            let mut digits = Vec::<char>::new();
            for n in 0..problems.len() {
                let c = match problem_strings[n][p].chars().nth(pos) {
                    Some(c) => c,
                    _ => continue
                };
                if c.is_digit(10) {
                    digits.push(c);
                }
            }
            let number = digits.iter().collect::<String>().parse::<i64>().expect("can't parse i64");
            numbers.push(number);
        }
        let mut res = numbers[0];
        for n in 1..numbers.len() {
            let number = numbers[n];
            let c = line.chars().nth(p).expect("no operation");
            match c {
                '*' => res *= number,
                '+' => res += number,
                _ => panic!("invalid operation {}", c)
            }
        }
        glob_sum += res;
    }
    println!("part 2: {}", glob_sum)
}
