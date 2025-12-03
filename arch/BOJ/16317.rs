use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = it.next().unwrap();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(it.next().unwrap());
    } 
    let mut nc = 0;
    let mut d = 0;
    let mut j = 0;
    let mut ans = 0;
    for i in 1..=365 {
        if j < n && arr[j] == i {
            nc += 1;
            j += 1;
        }
        d += nc;
        if d >= 20 {
            ans += 1;
            nc = 0;
            d = 0;
        }
    }
    if d != 0 {
        ans += 1;
    }
    println!("{}", ans);
}