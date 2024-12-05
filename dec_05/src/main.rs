use std::io;
use std::mem::swap;
use multimap::MultiMap;


fn read_maps() -> (MultiMap<i32,i32>, MultiMap<i32,i32>) {
    let mut map_f = MultiMap::<i32,i32>::new();
    let mut map_b = MultiMap::<i32,i32>::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let pair: Vec<i32> = input.split('|')
            .filter_map(|s| s.parse().ok()).collect();
        map_f.insert(pair[0],pair[1]);
        map_b.insert(pair[1],pair[0]);
    }
    (map_f, map_b)
}
fn read_lists() -> Vec<Vec<i32>> {
    let mut lists = Vec::<Vec<i32>>::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let list: Vec<i32> = input.split(',')
            .filter_map(|s| s.parse().ok()).collect();
        lists.push(list);
    }
    lists
}

fn order(list: &mut Vec<i32>, map_b: &MultiMap<i32,i32>){
    loop {
        let mut good = true;
        for i in 0..list.len(){
            for j in i+1..list.len() {
                if let Some(values) = map_b.get_vec(&list[i]) {
                    if values.contains(&list[j]) {
                        list.swap(i,j);
                        good = false;
                        break;
                    }
                }
            }
            if !good {
                break;
            }
        }
        if good {
            break;
        }
    }
}

fn main() {
    let (map_f, map_b) = read_maps();
    let lists = read_lists();
    // println!("{:?}",map);
    // println!("{:?}",lists);
    let mut sum = 0;
    for mut l in lists {
        let mut good = true;
        for i in 0..l.len(){
            for j in i+1..l.len(){
                if let Some(values) = map_b.get_vec(&l[i]) {
                    if values.contains(&l[j]){
                        good = false;
                        break;
                    }
                }
            }
            if !good{
                break;
            }
        }
        if !good {
            order(&mut l,&map_b);
            sum += l[l.len()/2];
        }
    }
    println!("{}",sum);
}
