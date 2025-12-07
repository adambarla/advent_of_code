use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        panic!("missing file name");
    }
    let file_name: &String = &args[1];
    let input = fs::read_to_string(file_name).unwrap();

    let n = 100;
    let mut pos: i32 = 50;
    let mut pass = 0;
    for line in input.split("\n") {
        let dir = match line.chars().nth(0) {
            Some(d) => d,
            None => break,
        };
        let num = &line[1..].parse::<i32>().unwrap();
        let mut step = 1;
        if dir == 'L' {
            step = -1;
        }
        for i in 0..num.abs() {
            let prev = pos;
            pos += step;
            if pos < 0 {
                pos += n;
            }
            if pos >= n {
                pos -= n;
            }
            if pos == 0 {
                pass += 1;
            }
        }
    }
    println!("{}", pass);
}
