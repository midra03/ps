use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let s: Vec<char> = it.next().unwrap().chars().collect();
    let l = s.len();
    let mut col = 0;
    let mut bar = 0;
    for i in 0..l {
        match s[i] {
            ':' => col += 1,
            '_' => bar += 1,
            _ => {}
        }
    }
    println!("{}", l + col + bar * 5);
}