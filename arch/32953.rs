use std::{collections::HashMap, io::{Read, stdin}};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let mut dict = HashMap::new();
    for _ in 0..n {
        let ki = it.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..ki {
            let s = it.next().unwrap().parse::<usize>().unwrap();
            dict.entry(s).and_modify(|key| *key += 1).or_insert(1);
        }
    }
    let mut ans = 0;
    for (_, v) in &dict {
        if *v >= m {
            ans += 1;
        }
    }
    println!("{}", ans);
}