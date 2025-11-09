use std::{collections::VecDeque, io::{Read, stdin}};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let mut d = [[-1; 599]; 599];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let x = it.next().unwrap().parse::<usize>().unwrap();
    let y = it.next().unwrap().parse::<usize>().unwrap();
    d[y][x] = 0;
    q.push_front((y, x));
    let dy = [-1, 1, -2, 2, -2, 2, -1, 1];
    let dx = [-2, -2, -1, -1, 1, 1, 2, 2];
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for i in 0..8 {
            let ty = u.0 as i32 + dy[i];
            let tx = u.1 as i32 + dx[i];
            if tx < 1 || tx > n as i32 || ty < 1 || ty > n as i32 {
                continue;
            }
            let ny = ty as usize;
            let nx = tx as usize;
            if d[ny][nx] != -1 {
                continue;
            } 
            d[ny][nx] = d[u.0][u.1] + 1;
            q.push_back((ny, nx));
        }
    }
    for _ in 0..m {
        let x1 = it.next().unwrap().parse::<usize>().unwrap();
        let y1 = it.next().unwrap().parse::<usize>().unwrap();
        print!("{} ", d[y1][x1]);
    }
    
    println!();
}