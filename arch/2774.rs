use std::{collections::BTreeSet, io::{Read, stdin}};

fn sol(n: i32) {
    let mut tmp = n;
    let mut s = BTreeSet::new();
    while tmp != 0 {
        s.insert(tmp % 10);
        tmp /= 10;
    }
    println!("{}", s.len());
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = it.next().unwrap().parse::<i32>().unwrap();
        sol(n);
    }
}