use std::io;
use regex::Regex;

fn read() -> Vec<Vec<char>> {
    let mut letters =  Vec::<Vec<char>>::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        letters.push(input.chars().collect());
    }
    letters
}


fn create_strings(letters: &Vec<Vec<char>>) -> Vec<String> {
    let mut strings = Vec::<String>::new();
    for k in 0..letters.len() {
        let mut s = String::new();
        for i in 0..letters[k].len() {
            s.push(letters[k][i]);
        }
        strings.push(s.clone());
        strings.push(s.chars().rev().collect());
    }
    for k in 0..letters.len() {
        let mut s = String::new();
        for i in 0..letters.len() {
            s.push(letters[i][k]);
        }
        strings.push(s.clone());
        strings.push(s.chars().rev().collect());
    }
    for k in 0..letters.len(){
        let mut s = String::new();
        for i in 0..(letters[0].len()-k) {
            s.push(letters[i+k][i]);
        }
        strings.push(s.clone());
        strings.push(s.chars().rev().collect());
    }
    for k in 1..letters[0].len(){
        let mut s = String::new();
        for i in 0..letters.len()-k {
            s.push(letters[i][i+k]);
        }
        strings.push(s.clone());
        strings.push(s.chars().rev().collect());
    }
    for k in 0..letters.len(){
        let mut s = String::new();
        for i in 0..(letters[0].len()-k) {
            s.push(letters[i+k][letters[0].len()-1-i]);
        }
        strings.push(s.clone());
        strings.push(s.chars().rev().collect());
    }
    for k in 1..letters[0].len(){
        let mut s = String::new();
        for i in 0..letters.len()-k {
            s.push(letters[i][letters[0].len() -1 -(i+k)]);
        }
        strings.push(s.clone());
        strings.push(s.chars().rev().collect());
    }
    strings
}

fn main() {
    let letters = read();
    let strings = create_strings(&letters);
    // println!("{}",strings.len()/2);
    // println!("{:?}", strings);
    let re = Regex::new(r"XMAS").unwrap();
    let mut count = 0;
    for s in strings {
        count += re.find_iter(&s).count();
    }
    println!("{}", count);
}
