use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        panic!("missing file name");
    }
    let file_name: &String = &args[1];
    let input = fs::read_to_string(file_name).unwrap();

    let mut pos = 50;
    let mut pass = 0;
    for line in input.split("\n") {
        // println!("{}", line);
        let dir = match line.chars().nth(0) {
            Some(d) => d,
            None => break
        };
        let num = &line[1..].parse::<i32>().unwrap();
        if dir == 'L' {
            pos = (pos - num + 100) % 100;
        } 
        else if dir == 'R' {
            pos = (pos + num + 100) % 100;
        }
        else {
            panic!("unknown direction")
        }
        if pos == 0 {
            pass += 1;
        }
        // println!("{}", pos)
    }
    println!("{}", pass);
}
