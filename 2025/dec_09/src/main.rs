use std::env;
use std::fs;

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
    let mut pairs = Vec::new();
    for line in file.lines() {
        let (a,b) = line.split_once(",").unwrap();
        pairs.push((a.parse::<i128>().unwrap(), b.parse::<i128>().unwrap()));
    }
    println!("{:?}", pairs);
    let mut max_area = 0;
    for i in 0..pairs.len() {
        for j in i+1..pairs.len() {
            let a = (pairs[i].0 - pairs[j].0).abs() + 1;
            let b = (pairs[i].1 - pairs[j].1).abs() + 1;
            let area = a * b;
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("part 1: {}", max_area);

}
