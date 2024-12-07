use std::io;

fn read() -> (Vec<Vec<usize>>, Vec<usize>){
    let mut a = Vec::<Vec<usize>>::new();
    let mut b = Vec::<usize>::new();
    // row x: y z ... is b:[a]

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let (bi,ai) = input
            .split_once(":")
            .expect("failed to split");
        a.push(ai
            .split_whitespace()
            .map(|x| x.parse().expect("failed to parse"))
            .collect());
        b.push(bi.parse().expect("failed to parse"));
    }
    (a,b)
}

fn test(i:usize, a: &Vec<usize>, b: usize,  sum_tmp:usize) -> bool {
    if i == a.len() {
        return sum_tmp == b;
    }
    test(i + 1, a, b, sum_tmp * a[i])
    || test(i + 1, a, b,sum_tmp+a[i])
}

fn main() {
    let (a,b) = read();
    // println!("{:?}",a);
    // println!("{:?}",b);
    let mut sum = 0;
    for i in 0..a.len() {
        // println!("{:?}",a[i]);
        // println!("{:?}",b[i]);
        if test(0,&a[i],b[i],0) {
            sum += b[i];
        }
    }
    println!("{}",sum);
}
