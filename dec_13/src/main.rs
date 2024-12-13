use std::cmp::min;
use std::collections::HashMap;
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

fn f(p: &(i64,i64), a: &(i64,i64), b: &(i64,i64), k:i64, l:i64, map: &mut HashMap<(i64,i64),u64>) -> u64 {
    let s = (p.0 - k * a.0 - l * b.0, p.1 - k * a.1 - l * b.1);
    if let Some(&v) = map.get(&s) {
        return v;
    }
    if s.0 < 0 || s.1 < 0 {
        map.insert(s, u64::MAX);
        return u64::MAX;
    }
    if s.0 == 0 && s.1 == 0 {
        let res = (3*k + l) as u64;
        map.insert(s,res);
        return res;
    }
    let res = min(f(p, a, b, k + 1, l, map),
        f(p, a, b, k, l + 1, map));
    map.insert(s, res);
    return res;
}

// fn f_lin(p: &(i64,i64), a: &(i64,i64), b: &(i64,i64)) -> u64 {
//     let t = Vec::<(i64,i64)>::new();
//     // linear version of f_r (recursive)
//     for k in 0.. {
//         for l in 0.. {
//             let s = (p.0 - k * a.0 - l * b.0, p.1 - k * a.1 - l * b.1);
//             if s.0 < 0 || s.1 < 0 {
//                 break;
//             }
//             if s.0 == 0 && s.1 == 0 {
//                 return (3*k + l) as u64;
//             }
//         }
//     }
//     u64::MAX
//
// }

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s*Button B: X\+(\d+), Y\+(\d+)\s*Prize: X=(\d+), Y=(\d+)"
    ).unwrap();

    let mut sum: u64 = 0;
    let n = 10000000000000i64;
    for (i,cap) in re.captures_iter(&input).enumerate() {
        let a_x = cap[1].parse::<i64>().unwrap();
        let a_y = cap[2].parse::<i64>().unwrap();
        let b_x = cap[3].parse::<i64>().unwrap();
        let b_y = cap[4].parse::<i64>().unwrap();
        let p_x = cap[5].parse::<i64>().unwrap() + n;
        let p_y = cap[6].parse::<i64>().unwrap() + n;
        // find if p can be made as a linear combination of a and b
        // where the coefficients are POSITIVE INTEGERS
        println!("Case {}: ", i + 1);
        let det = a_x * b_y - a_y * b_x;
        let mut k;
        let mut l;
        if det != 0 {
            let k_num = p_x * b_y - p_y * b_x;
            let l_num = a_x * p_y - a_y * p_x;
            if k_num % det != 0 || l_num % det != 0 {
                println!("Impossible (det)");
                continue;
            }
            k = k_num / det;
            l = l_num / det;
            if k < 0 || l < 0 {
                println!("Impossible (negative solution)");
                continue;
            }
            println!("k: {}, l: {}", k, l);
            println!("{} ?= {}",k*a_x + l*b_x, p_x);
            sum += (3 * k + l) as u64;
        }
        else {
            let g_x;
            let g_y;
            (g_x, k, l) = extended_gcd(a_x, b_x);
            (g_y, _, _) = extended_gcd(a_y, b_y);
            if p_x % g_x != 0 || p_y % g_y != 0 {
                println!("Impossible (gcd)");
                continue;
            }
            let a_x_ = a_x / g_x;
            let b_x_ = b_x / g_x;
            let p_x_ = p_x / g_x;

            let k_0 = k * p_x_;
            let l_0 = l * p_x_;

            let num = p_y - k_0 * a_y - l_0 * b_y;
            let den = b_x_ * a_y - a_x_ * b_y;
            if den == 0 {
                if num != 0 {
                    println!("Impossible (den=0)");
                    continue;
                }
                let t_min = -(k_0 as f64 / b_x_ as f64).ceil() as i64;
                let t_max = (l_0 as f64 / a_x_ as f64).floor() as i64;
                if t_min > t_max {
                    println!("Impossible (non-negative solution)");
                    continue;
                }
                println!("t_min: {}, t_max: {}", t_min, t_max);
                let mut t;
                // if (3*b_x_ - a_x_) > 0 {
                //     t = t_min;
                // } else {
                //     t = t_max;
                // }
                t = t_max;
                k = k_0 + t * b_x_;
                l = l_0 - t * a_x_;
                println!("k: {}, l: {}", k, l);
                println!("{} ?= {}",k*a_x + l*b_x, p_x);
                sum += (3 * k + l) as u64;

            }
            else {
                if num % den != 0 {
                    println!("Impossible (num)");
                    continue;
                }
                let t = num / den;
                k = k_0 + t * b_x_;
                l = l_0 - t * a_x_;
                if k < 0 || l < 0 {
                    println!("Impossible (negative solution)");
                    continue;
                }
                println!("k: {}, l: {}", k, l);
                println!("{} ?= {}",k*a_x + l*b_x, p_x);
                sum += (3 * k + l) as u64;
            }
        }





        // let mut sum_tmp = u64::MAX;
        // for t in t_min..=t_max {
        //     let k_x = k_x_0 + t * b_x_;
        //     let l_x = l_x_0 - t * a_x_;
        //     println!("k_x: {}, l_x: {}", k_x, l_x);
        //     println!("{} ?= {}",k_x*a_x + l_x*b_x, p_x);
        //     println!("{} ?= {}",k_x*a_y + l_x*b_y, p_y);
        //     if p_x == k_x * a_x + l_x * b_x && p_y == k_x * a_y + l_x * b_y {
        //         let tmp = (3 * k_x + l_x) as u64;
        //         if tmp < sum_tmp {
        //             sum_tmp = tmp;
        //         }
        //     }
        // }
        // if sum_tmp == u64::MAX {
        //     println!("Impossible");
        // } else {
        //     println!("Minimum size: {}", sum_tmp);
        //     sum += sum_tmp;
        // }


        // let t1 = if k_x_0 < 0 {-k_x_0 / b_x_1} else {0};
        // let t2 = if l_x_0 > 0 {l_x_0 / a_x_1} else {0};
        // let t = min(t1, t2);
        // let t_min = if b_x_1 > 0 { -(k_x_0 as f64 / b_x_1 as f64).ceil() as i64} else { 0 };
        // let t_max = if a_x_1 > 0 { (l_x_0 as f64 / a_x_1 as f64).floor() as i64} else { 0 };
        // let mut t = 0;
        // if t_min > t_max {
        //     println!("Impossible (non-negative solution)");
        //     continue;
        // }
        // if (3*b_ - a_) > 0{
        //     t = t_min
        // }
        // else:
        // # Function decreases with t, choose t = t_max
        // t_opt = t_max
        // let mut t = t_min;
        //
        // let mut k = k_x_0;
        // let mut l = l_x_0;
        // // let mut t = 0;
        // while k/p_x_1 < 0 || l/p_x_1 < 0 {
        //     t -= 1;
        //     println!("t: {}", t);
        //     k = k_x_0 + t * b_x_1;
        //     l = l_x_0 - t * a_x_1;
        //     println!("k: {}, l: {}", k, l);
        //     println!("{} ?= {}",k*a_x_1 + l*b_x_1, p_x_1);
        // }
        //


        // let res = f(&(p_x, p_y), &(a_x, a_y), &(b_x, b_y), 0, 0, &mut HashMap::new());
        // if res == u64::MAX {
        //     println!("Impossible");
        // } else {
        //     println!("Minimum size: {}", res);
        //     sum += res as u64;
        // }

    }
    println!("Total minimum size: {}", sum);

}

