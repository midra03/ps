use std::{cmp::min, collections::{BinaryHeap}, io::{Read, stdin}};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n= it.next().unwrap().parse::<usize>().unwrap();
    let mut p = it.next().unwrap().parse::<usize>().unwrap();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let ai = it.next().unwrap().parse::<usize>().unwrap();
        a.push(ai);
    }
    for _ in 0..n {
        let bi = it.next().unwrap().parse::<usize>().unwrap();
        b.push(bi);
    }
    let mut sum = 0;
    for i in 0..n {
        sum += a[i] + b[i];
    }
    sum *= n;
    let mut abit = vec![0; 27];
    for i in 0..n {
        for j in 0..27 {
            if (a[i] & (1 << j)) != 0 {
                abit[j] += 1;
            }
        }
    }
    let mut ab = 0;
    for i in 0..n {
        for j in 0..27 {
            if (b[i] & (1 << j)) != 0 {
                ab += abit[j] * (1 << j);
            }
        }
    }
    sum -= 2 * ab;
    let mut acnt = [0; 1027];
    let mut bcnt = [0; 1027];
    for i in 0..a.len() {
        acnt[a[i]] += 1;
    }
    for i in 0..b.len() {
        bcnt[b[i]] += 1;
    }
    let mut base = BinaryHeap::new();
    for i in 0..1027 {
        for j in 0..1027 {
            base.push((i & j , (i, j)));
        }
    }
    while p > 0 || !base.is_empty() {
        let tmp= base.pop().unwrap();
        let val = tmp.0;
        let ai = tmp.1.0;
        let bi = tmp.1.1;
        let cnt = acnt[ai] * bcnt[bi];
        sum += val * min(cnt, p);
        p -= min(cnt, p);
    }
    println!("{}", sum);
}