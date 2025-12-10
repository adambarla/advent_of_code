use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;
use bit_set::BitSet;
use rand::Rng;


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

fn get_area(ps: &Vec<(i32, i32)>, i: usize, j: usize) -> i64 {
    let a = (ps[i].0 - ps[j].0).abs() + 1;
    let b = (ps[i].1 - ps[j].1).abs() + 1;
    return a as i64 * b as i64;
}

fn is_on_line(ps: &Vec<(i32, i32)>, lines: &Vec<(usize, usize)>, pt: (i32, i32)) -> bool {
    for line in lines {
        let (i, j) = line;
        let (x1, y1) = ps[*i];
        let (x2, y2) = ps[*j];
        // check if the point is on the line (x1, y1) -- (x2, y2)
        if x1 == x2 && pt.0 == x1 && pt.1 >= y1.min(y2) && pt.1 <= y1.max(y2) {
            return true;
        }
        if y1 == y2 && pt.1 == y1 && pt.0 >= x1.min(x2) && pt.0 <= x1.max(x2) {
            return true;
        }
    }
    return false;
}

fn is_valid_rectangle(valid_pts: &Vec<BitSet>, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    
    if !valid_pts[y1 as usize].contains(x2 as usize) 
    || !valid_pts[y2 as usize].contains(x1 as usize) {
        return false;
    }
    // get random point in the rectangle K times and check if it is valid
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = rng.gen_range(min_x..max_x+1);
        let y = rng.gen_range(min_y..max_y+1);
        if !valid_pts[y as usize].contains(x as usize) {
            return false;
        }
    }
    // check all points in the rectangle
    for x in min_x..max_x+1 {
        for y in min_y..max_y+1 {
            if !valid_pts[y as usize].contains(x as usize) {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let file = get_file();
    let mut ps = Vec::new();
    for line in file.lines() {
        let (a,b) = line.split_once(",").unwrap();
        let x = a.parse::<i32>().unwrap();
        let y = b.parse::<i32>().unwrap();
        ps.push((x, y));
    }
    // println!("{:?}", ps);
    let mut max_area = 0;
    for i in 0..ps.len() {
        for j in i+1..ps.len() {
            let area = get_area(&ps, i, j);
            if area > max_area {
                max_area = area;
            }   
        }
    }
    println!("part 1: {}", max_area);
    // define all lines
    let mut ps_set: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..ps.len() {
        ps_set.insert((ps[i].0, ps[i].1));
    }
    let mut lines: Vec<(usize, usize)> = Vec::new();
    let n = ps.len();
    for i in 1..n {
        lines.push((i-1, i));
    }
    lines.push((n-1, 0));

    // for a grid 20x20 mark all points on the line as X and points in pts as # else .
    // print the grid
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for pt in &ps {
        min_x = min_x.min(pt.0);
        min_y = min_y.min(pt.1);
        max_x = max_x.max(pt.0);
        max_y = max_y.max(pt.1); 
    }
    println!("min_x: {}, min_y: {}, max_x: {}, max_y: {}", min_x, min_y, max_x, max_y);

    // try to load from binary file
    let mut valid_pts: Vec<BitSet> = Vec::new();
    if valid_pts.is_empty() {
        'start_search: for (start_dx, start_dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            println!("start_dx: {}, start_dy: {}", start_dx, start_dy);
            valid_pts.clear();
            for _ in 0..=max_y+1 {
                valid_pts.push(BitSet::with_capacity((max_x + 2) as usize));
            }
            // inclute all points on the lines
            for line in &lines {
                let (i, j) = line;
                let (x1, y1) = ps[*i];
                let (x2, y2) = ps[*j];
                for x in x1.min(x2)..x1.max(x2)+1 {
                    for y in y1.min(y2)..y1.max(y2)+1 {
                        valid_pts[y as usize].insert(x as usize);
                    }
                }
            }
            println!("valid_pts: {}", valid_pts.len());
            let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
            // start somewhere under or right of the first line
            let (x, y) = lines[0];
            let (x1, y1) = ps[x];
            let (x2, y2) = ps[y];
            if x1 == x2 {
                let p = (x1+start_dx, (y1+y2)/2+start_dy);
                queue.push_back(p);
                valid_pts[p.1 as usize].insert(p.0 as usize);
            } else {
                let p = ((x1+x2)/2+start_dx, y2+1+start_dy);
                queue.push_back(p);
                valid_pts[p.1 as usize].insert(p.0 as usize);
            }

            let mut count : i128 = 0;
            let max_count = ((max_x + 2) as i128) * ((max_y + 2) as i128);
            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();
                // insert neighbours
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < min_x || ny < min_y || nx > max_x || ny > max_y {
                        println!("out of bounds: ({}, {})", nx, ny);
                        continue 'start_search;
                    }
                    if valid_pts[ny as usize].contains(nx as usize) {
                        continue;
                    }
                    queue.push_back((nx, ny));
                    valid_pts[ny as usize].insert(nx as usize);
                }
                if count % 100000000 == 0 {
                    println!("progress: {:.2}%", (count as f64 / max_count as f64) * 100.0);
                }
                count += 1;
            }
            break
        }
    }


    // for j in min_y-1..max_y+2 {
    //     for i in min_x-1..max_x+2 {
    //         if valid_pts[j as usize].contains(i as usize) {
    //             print!("#");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     println!();
    // }

    let mut max_area = 0;
    for i in 0..ps.len() {
        for j in i+1..ps.len() {
            // check if the oposite corner is in the points or lies on some line
            let (x1, y1) = ps[i];
            let (x2, y2) = ps[j];
            if !is_valid_rectangle(&valid_pts, x1, y1, x2, y2) {
                continue;
            }
            let area = get_area(&ps, i, j);
            if area > max_area {
                max_area = area;
            }   
        }
    }
    println!("part 2: {}", max_area);
}
