use std::io;
use std::io::Read;
use regex::Regex;

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}

fn main() {
    let input = read();
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let mut sum =  0;
    let mut enabled = true;
    for cap in re.captures_iter(&input) {
        // println!("Found match: {}", &cap[0]);
        if &cap[0] == "do()" {
            enabled = true;
            continue;
        }
        if &cap[0] == "don't()" {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }
        let pair: Vec<i32> = cap[0]
            .split(",")
            .map(|s| s.trim_matches(|p| p == 'm' || p == 'u' || p == 'l' || p == '(' || p == ')'))
            .map(|s| s.parse().unwrap())
            .collect();
        if pair.len() != 2 {
            panic!("Error parsing pair");
        }
        sum += pair[0] * pair[1];

    }
    println!("Sum: {}", sum);
}
