use std::io::{self, Read};
use regex::Regex;

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r_0 = a;
    let mut r_1 = b;
    let mut s_0 = 1;
    let mut s_1 = 0;
    let mut t_0 = 0;
    let mut t_1 = 1;
    while r_1 != 0 {
        let q = r_0 / r_1;
        let r = r_0 % r_1;
        let s = s_0 - q * s_1;
        let t = t_0 - q * t_1;
        r_0 = r_1;
        r_1 = r;
        s_0 = s_1;
        s_1 = s;
        t_0 = t_1;
        t_1 = t;
    }
    (r_0, s_0, t_0) // g, k, l such that g = a * k + b * l
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s*Button B: X\+(\d+), Y\+(\d+)\s*Prize: X=(\d+), Y=(\d+)"
    ).unwrap();

    let mut sum: u64 = 0;
    let n = 10000000000000i64;
    for cap in re.captures_iter(&input) {
        let a_x = cap[1].parse::<i64>().unwrap();
        let a_y = cap[2].parse::<i64>().unwrap();
        let b_x = cap[3].parse::<i64>().unwrap();
        let b_y = cap[4].parse::<i64>().unwrap();
        let p_x = cap[5].parse::<i64>().unwrap() + n; // remove +n for the first part
        let p_y = cap[6].parse::<i64>().unwrap() + n;
        let det = a_x * b_y - a_y * b_x;
        if det != 0 { // dependent
            let k_num = p_x * b_y - p_y * b_x;
            let l_num = a_x * p_y - a_y * p_x; // cramers rule
            if k_num % det != 0 || l_num % det != 0 {
                continue; // no integer solution
            }
            let k = k_num / det;
            let l = l_num / det;
            if k < 0 || l < 0 {
                continue; // negative solution
            }
            sum += (3 * k + l) as u64;
        }
        else {
            let (g_x, k, l) = extended_gcd(a_x, b_x);
            let (g_y, _, _) = extended_gcd(a_y, b_y);
            if p_x % g_x != 0 || p_y % g_y != 0 {
                continue; // no integer solution
            }
            let a_x_ = a_x / g_x;
            let b_x_ = b_x / g_x;
            let p_x_ = p_x / g_x;

            let k_0 = k * p_x_;
            let l_0 = l * p_x_;
            // general solution to the first equation
            // k = k_0 + t * b_x_ and l = l_0 - t * a_x_
            // plugging in the equation for k and l into second equation
            // p_y = k * a_y + l * b_y
            // p_y = (k_0 + t * b_x_) * a_y + (l_0 - t * a_x_) * b_y
            // rearranging for t
            // t = (p_y - k_0 * a_y - l_0 * b_y) / (b_x_ * a_y - a_x_ * b_y)
            let num = p_y - k_0 * a_y - l_0 * b_y;
            let den = b_x_ * a_y - a_x_ * b_y;
            if den == 0 {
                if num != 0 {
                    continue; // no solution
                }
                let t_min = -(k_0 as f64 / b_x_ as f64).ceil() as i64;
                let t_max = (l_0 as f64 / a_x_ as f64).floor() as i64;
                if t_min > t_max {
                    continue; // no non-negative solution
                }
                let t = t_max;
                let k = k_0 + t * b_x_;
                let l = l_0 - t * a_x_;
                sum += (3 * k + l) as u64;
            }
            else {
                if num % den != 0 {
                    continue; // no integer solution
                }
                let t = num / den;
                let k = k_0 + t * b_x_;
                let l = l_0 - t * a_x_;
                if k < 0 || l < 0 {
                    continue; // negative solution
                }
                sum += (3 * k + l) as u64;
            }
        }
    }
    println!("{}", sum);
}

