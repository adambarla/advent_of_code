use std::cmp::min;
use std::collections::LinkedList;
use std::io;

#[derive(Debug)]
struct Block {
    id: u32,
    len: u32,
    size: u32,
}

fn main() {
    let mut ll = LinkedList::<Block>::new();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read");
    let s = s.trim();
    let ch: Vec<u32> = s.chars().map(|s| s.to_digit(10).ok_or("failed to parse")).collect::<Result<_, _>>().unwrap();
    let mut id = 0;
    for i in (0..ch.len()).step_by(2) {
        ll.push_back(Block {
            id,
            len: ch[i],
            size: ch[i] + (if i+1 < ch.len() {ch[i + 1]} else {0}),
        });
        id += 1;
    }
    let mut vec = Vec::<u32>::new();
    let mut i = 0u32;
    for _ in 0..ll.front().unwrap().len {
        vec.push(ll.front().unwrap().id * i);
        i += 1;
    }
    let mut sum: u64 = 0;
    loop {
        // println!("{:?}", ll.len());
        // println!("{:?}", ll);
        // println!("{:?} {:?}", ll.front().unwrap(), ll.back().unwrap());
        if ll.front().unwrap().size - ll.front().unwrap().len == 0 {
            ll.pop_front();
            for _ in 0..ll.front().unwrap().len {
                // vec.push(ll.front().unwrap().id * i);
                sum += (ll.front().unwrap().id * i) as u64;
                i += 1;
            }
        }
        if ll.back().unwrap().len == 0 {
            ll.pop_back();
        }
        if ll.len() == 1 {
            break;
        }
        let space = ll.front().unwrap().size - ll.front().unwrap().len;
        let fill = ll.back().unwrap().len;

        for _ in 0..min(space, fill) {
            // vec.push(ll.back().unwrap().id * i);
            sum += (ll.back().unwrap().id * i) as u64;
            i += 1;
        }
        ll.front_mut().unwrap().len += min(space, fill);
        ll.back_mut().unwrap().len -= min(space, fill);
    }

    println!("{:?}", sum);


    println!("{:?}", vec);
}
