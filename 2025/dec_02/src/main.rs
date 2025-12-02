use std::env;
use std::fs;

fn main() {
    // read the file
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        panic!("missing file name");
    }
    let file_name: &String = &args[1];
    let input = fs::read_to_string(file_name).unwrap();

    // we have the file
    let mut counter: i64 = 0;
    for range_str in input.split(",") {
        let res: Result<Vec<i64>, _> = range_str.split("-")
            .map(|s| s.trim().parse::<i64>())
            .collect();
        let range = match res {
            Ok(numbers) => numbers,
            Err(e) => {
                eprintln!("{}",e);
                break;
            }
        };
        println!("{:?}",range);
        for i in range[0]..range[1] + 1 {
            let s = i.to_string();
            // prinln!("{}", ln(s.len()))
            if s.len() % 2 != 0 {
                continue;
            }
            let (s_0, s_1) = s.split_at(s.len()/2);
            if s_0 == s_1 {
                // println!("{}", s);
                counter += i;
            }
        }
    }
    println!("{}",counter);
}
