use std::collections::HashSet;
use multimap::MultiMap;
use std::io;

fn read() -> (MultiMap<char,(f32, f32)>,i32){
    let mut map = MultiMap::<char,(f32,f32)>::new();
    let mut i = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let input = input.trim();
        if input.is_empty(){
            break;
        }
        for (j,c) in input.chars().enumerate(){
            if c == '.'{
                continue
            }
            map.insert(c,(i as f32, j as f32));
        }
        i += 1;
    }
    (map,i)
}

fn is_in(p: (f32,f32),n:i32) -> bool {
    let p = (p.0 as i32, p.1 as i32);
    if p.0 >= 0 && p.0 < n && p.1 >= 0 && p.1 < n {
        return true;
    }
    false
}

fn is_valid(p: (f32,f32)) -> bool {
    let tol = 1e-3;
    let q = ((p.0 - ((p.0 as i32) as f32)),(p.1 - ((p.1 as i32) as f32)));
    if q.0.abs() > tol || q.1.abs() > tol {
        return false;
    }
    true
}

fn main() {
    let mut set = HashSet::<(i32,i32)>::new();
    let (map,n) = read();
    for (_, ps) in &map{
        for i in 0..ps.len(){
            for j in i+1..ps.len(){
                let p1 = ps[i];
                let p2 = ps[j];
                set.insert((p1.0 as i32,p1.1 as i32));
                set.insert((p2.0 as i32,p2.1 as i32));
                let v = (p1.0 - p2.0, p1.1 - p2.1);
                let mut k = 1.0;
                loop {
                    let p = (p1.0 + k*v.0, p1.1 + k*v.1);
                    if !is_in(p,n){
                        break;
                    }
                    if is_valid(p){
                        let p = (p.0 as i32, p.1 as i32);
                        set.insert(p);
                    }
                    k += 1.0;
                }
                k = 1.0;
                loop {
                    let p = (p2.0 - k*v.0, p2.1 - k*v.1);
                    if !is_in(p,n){
                        break;
                    }
                    if is_valid(p){
                        let p = (p.0 as i32, p.1 as i32);
                        set.insert(p);
                    }
                    k += 1.0;
                }
                k = 3.0;
                loop {
                    let p = (p1.0 - v.0/k, p1.1 - v.1/k);
                    if (p1.0 - p.0).abs() < 1.0 || (p1.1 - p.1).abs() < 1.0{
                        break
                    }
                    if is_valid(p){
                        let p = (p.0 as i32, p.1 as i32);
                        set.insert(p);
                    }
                    k += 1.0;
                }
                k = 3.0;
                loop {
                    let p = (p2.0 + v.0/k, p2.1 + v.1/k);
                    if (p2.0 - p.0).abs() < 1.0 || (p2.1 - p.1).abs() < 1.0{
                        break
                    }
                    if is_valid(p){
                        let p = (p.0 as i32, p.1 as i32);
                        set.insert(p);
                    }
                    k += 1.0;
                }
            }
        }
    }
    // for i in 0..n{
    //     for j in 0..n{
    //         if set.contains(&(i,j)){
    //             print!("#")
    //         }
    //         else{
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }
    // println!("{:?}", set);
    println!("{:?}", set.len());
}
