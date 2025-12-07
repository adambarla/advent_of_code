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
    let lines = file.split('\n').collect::<Vec<&str>>();
    let mut l = 0;
    let mut length = 0;
    let mut problems : Vec<Vec<i64>> = Vec::<Vec<i64>>::new();
    loop {
        let line = lines[l]; 
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
    println!("{:?}", problems);
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
                other => panic!("invalid operation {}", c)
            }
        }
        glob_sum += res;
        print!("{} ", res);
    }
    println!();
    println!("{}", glob_sum)
}
