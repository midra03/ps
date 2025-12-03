use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut ans = 0;
    loop {
        let m = it.next().unwrap().parse::<i32>().unwrap();
        let w = it.next().unwrap().parse::<i32>().unwrap();
        if m == 0 && w == 0 {
            break;
        }
        println!("{}", m + w);
    }
}