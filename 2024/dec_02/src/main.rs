use std::io;

fn read() -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let report: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        reports.push(report);
    }
    reports
}


fn sign(x: i32) -> i32
{
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut dir = 0;
    let mut safe = true;
    let mut i = 0;
    let mut prev = report[i];
    loop {
        i += 1;
        if i == report.len() {
            break;
        }
        let curr = report[i];
        let diff = curr - prev;
        if diff.abs() < 1 || diff.abs() > 3 {
            safe = false;
            break
        }
        if dir == 0 {
            dir = sign(diff);
        }
        else if dir != sign(diff) {
            safe = false;
            break;
        }
        prev = curr;
    }
    safe
}

fn tol_is_safe(report: &Vec<i32>, skip:usize) -> bool {
    let mut dir = 0;
    let mut safe = true;
    let mut i = if skip != 0 {0} else {1};
    let mut prev = report[i];
    loop {
        i += 1;
        if i == report.len() {
            break;
        }
        if i == skip {
            continue;
        }
        let curr = report[i];
        let diff = curr - prev;
        if diff.abs() < 1 || diff.abs() > 3 {
            safe = false;
            break
        }
        if dir == 0 {
            dir = sign(diff);
        }
        else if dir != sign(diff) {
            safe = false;
            break;
        }
        prev = curr;
    }
    safe
}

fn count_safe(reports: &Vec<Vec<i32>>) -> i32 {
    if reports.len() == 0 {
        return 0;
    }
    let mut count = 0;
    for report in reports {
        if report.len() == 1 {
            count += 1;
        }
        if is_safe(report) {
            count += 1;
        }
    }
    count
}

fn tol_count_safe(reports: &Vec<Vec<i32>>) -> i32 {
    if reports.len() == 0 {
        return 0;
    }
    let mut count = 0;
    for report in reports {
        if report.len() == 1 {
            count += 1;
        }
        for i in 0..report.len() {
            if tol_is_safe(report, i) {
                count += 1;
                break;
            }
        }

    }
    count
}

fn main() {
    let reports = read();
    let num = count_safe(&reports); //*
    println!("{}", num);
    let num = tol_count_safe(&reports);
    println!("{}", num);
}
