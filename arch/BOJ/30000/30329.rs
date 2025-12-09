use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let s: Vec<char> = it.next().unwrap().chars().collect();
    let n = s.len();
    let mut ans = 0;
    for i in 0..(n - 3) {
        if s[i] == 'k' && s[i + 1] == 'i' && s[i + 2] == 'c' && s[i + 3] == 'k' {
            ans += 1;
        }
    }
    println!("{}", ans);
}