use std::collections::HashSet;
use std::fs;
use std::process::Command;
use rand::Rng;

#[test]
fn test_unique_ids() {
    let mut rng = rand::thread_rng();
    let mut all_numbers = HashSet::<u64>::new();
    let mut ranges = Vec::<(u64, u64)>::new();
    
    let num_ranges = 100;
    for _ in 0..num_ranges {
        let start = rng.gen_range(1..=1000);
        let end = rng.gen_range(1..=1000);
        
        for num in start..=end {
            all_numbers.insert(num);
        }
        
        ranges.push((start, end));
    }
    
    let mut file_content = String::new();
    for (start, end) in &ranges {
        file_content.push_str(&format!("{}-{}\n", start, end));
    }
    file_content.push('\n');
    file_content.push_str("1\n");
    
    let test_file = "test_01";
    fs::write(test_file, file_content).expect("Failed to write test file");
    
    let output = Command::new("cargo")
        .args(&["run", "--", test_file])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let expected_count = all_numbers.len() as u64;
    
    let num_ids_line = stdout
        .lines()
        .find(|line| line.starts_with("num ids:"))
        .expect("Could not find 'num ids:' in output");
    
    let actual_count: u64 = num_ids_line
        .split_whitespace()
        .nth(2)
        .expect("Could not parse count")
        .parse()
        .expect("Count is not a number");
    
    assert_eq!(actual_count, expected_count, 
        "Expected {} unique IDs, but got {}", expected_count, actual_count);
    
    println!("{} {}",actual_count, expected_count);
    fs::remove_file(test_file).ok();
}

