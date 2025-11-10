use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let mut arr = vec![];
    for _ in 0..(n + m) {
        let t = it.next().unwrap().parse::<i32>().unwrap();
        arr.push(t);
    }
    arr.sort_by(|a, b| a.cmp(b));
    for i in arr {
        print!("{} ", i);
    }
    println!();
}