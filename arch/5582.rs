use std::{cmp::max, io::{Read, stdin}};

fn sol(memo: &mut Vec<Vec<i32>>, a: &[u8], b: &[u8], i: usize, j: usize) -> i32 {
    if i >= a.len() || j >= b.len() {
        return 0;
    }
    if memo[i][j] != -1 {
        return memo[i][j];
    }
    let mut ret = 0;
    if a[i] == b[j] {
        ret = sol(memo, a, b, i + 1, j + 1) + 1;
    }
    memo[i][j] = ret;
    return ret;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let a = it.next().unwrap().as_bytes();
    let b = it.next().unwrap().as_bytes();
    let mut memo = vec![vec![-1; b.len() + 1]; a.len() + 1];
    let mut ans = 0;
    for i in 0..a.len() {
        for j in 0..b.len() {
            ans = max(ans, sol(&mut memo, a, b, i, j));
        }
    }
    println!("{}", ans);
}