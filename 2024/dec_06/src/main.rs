use std::collections::HashSet;
use std::io;

fn read() -> (Vec<Vec<i32>>,(i32,i32),(i32,i32)) {
    let mut map = Vec::<Vec<i32>>::new();
    let mut player = (0,0);
    let mut dir = (0,0);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        let row: Vec<i32> = input
            .chars()
            .map(|c| {
                match c {
                    '#' => 1,
                    '.' => 0,
                    '^' => {
                        dir = (-1,0);
                        2
                    },
                    '<' => {
                        dir = (0,-1);
                        2
                    },
                    '>' => {
                        dir = (0,1);
                        2
                    },
                    'v' => {
                        dir = (1,0);
                        2
                    },
                    _ => 0
                }
            })
            .collect();
        if let Some(col) = row.iter().position(|&x| x == 2) {
            player = (map.len() as i32, col as i32);
        }
        map.push(row);
    }
    (map, player,dir)
}

fn move_player(map: &Vec<Vec<i32>>, player: (i32,i32), dir: (i32,i32)) -> ((i32,i32),(i32,i32)) {
    let mut new_dir = dir;
    loop{
        let new_player = (player.0 + new_dir.0, player.1 + new_dir.1);
        if new_player.0 < 0 || new_player.0 >= map.len() as i32 || new_player.1 < 0 || new_player.1 >= map[0].len() as i32 {
            return (new_player,dir);
        }
        if map[new_player.0 as usize][new_player.1 as usize] != 1 {
            return (new_player,new_dir);
        }
        new_dir = (new_dir.1,-new_dir.0);
        // println!("change({:?} {:?})", player, new_dir);
    }
}

fn play(map: &Vec<Vec<i32>>, player: (i32,i32), dir: (i32,i32)) -> (HashSet<((i32,i32),(i32,i32))>,i32) {
    let mut pos_set: HashSet<((i32,i32),(i32,i32))> = HashSet::new();
    let mut nplayer = player;
    let mut ndir = dir;
    loop{
        (nplayer,ndir) = move_player(&map, nplayer, ndir);
        if nplayer.0 < 0 || nplayer.0 >= map.len() as i32 || nplayer.1 < 0 || nplayer.1 >= map[0].len() as i32 {
            break;
        }
        if pos_set.contains(&(nplayer,ndir)) {
            return (pos_set,1);
        }
        if map[nplayer.0 as usize][nplayer.1 as usize] == 1 {
            ndir = (ndir.1,-ndir.0);
        }
        pos_set.insert((nplayer,ndir));
    }
    (pos_set,0)
}

fn main() {
    let (mut map, player, dir) = read();
    // println!("{:?}", map);
    // println!("{:?} {:?}", player, dir);
    // print!("{}",map[0].len());
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != 0 {
                continue;
            }
            map[i][j] = 1;
            let (_,looped) = play(&map, player, dir);
            if looped == 1 {
                count += 1;
                println!("{:5}/{:5}",j + i * map[0].len(), map.len() * map[0].len());
            }
            map[i][j] = 0
        }
    }
    println!("{}", count);
}
