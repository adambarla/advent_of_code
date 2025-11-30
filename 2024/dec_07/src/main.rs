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

fn do_op(a:usize, b:usize, op:i32) -> usize {
    match op {
        0 => a + b,
        1 => a * b,
        2 => {
            let mut tmp = b;
            let mut m = 1;
            while tmp > 0 {
                m *= 10;
                tmp /= 10;
            }
            m * a + b
        },
        _ => 0
    }
}

fn test(i:usize, a: &Vec<usize>, b: usize,  sum:usize, last_op:i32, tmp_number:usize) -> bool {
    if i == a.len() {
        if last_op == -1 {
            return sum == b;
        }
        return do_op(sum, tmp_number, last_op) == b;
    }
    if last_op == -1 {
        return test(i + 1, a, b, sum, 0, a[i])
            || test(i + 1, a, b, sum, 1, a[i])
            || test(i + 1, a, b, do_op(sum,a[i],2), -1, 0);
    }
    test(i + 1, a, b, do_op(sum, tmp_number, last_op), 0, a[i])
        || test(i + 1, a, b, do_op(sum, tmp_number, last_op), 1, a[i])
        || test(i + 1, a, b, do_op(sum, tmp_number, last_op), 2, a[i])
}

fn main() {
    let (a,b) = read();
    let mut sum = 0;
    for i in 0..a.len() {
        if test(1, &a[i], b[i], a[i][0], -1, 0) {
            sum += b[i];
        }
    }
    println!("{}",sum);
}
