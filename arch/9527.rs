use std::io::{stdin, Read};

fn count(cnt: &Vec<i64>, bit: i64, lo: i64, hi: i64, le: i64, ri: i64) -> (i64, i64) {
    if hi < le || lo > ri {
        return (0, 0);
    }
    if le <= lo && hi <= ri {
        return (1 << bit, cnt[bit as usize]);
    }
    let mid = (lo + hi) / 2;
    let lres = count(cnt, bit - 1, lo, mid, le, ri);
    let rres = count (cnt, bit - 1, mid + 1, hi, le, ri);
    return (lres.0 + rres.0, lres.1 + rres.0 + rres.1); 
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let a = it.next().unwrap().parse::<i64>().unwrap();
    let b = it.next().unwrap().parse::<i64>().unwrap();
    let mut cnt = vec![0i64];
    for i in 1..57 {
        cnt.push(cnt.last().unwrap() * 2 + (1i64 << (i - 1)));
    }
    println!("{}", count(&cnt, 57, 0, (1 << 57) - 1, a, b).1);
}