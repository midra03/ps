use std::{cmp::min, io::{Read, Write, stdin}};

fn lb<F: Fn(u32) -> bool>(f: F, le: u32, ri: u32) -> u32 { // 닫힌 구간
    let mut lo = le - 1;
    let mut hi = ri + 1;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if f(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    return hi;
}

fn g(n: u32, x: u32) -> u32 {
    let mut ans = 0;
    for i in 1..=n {
        ans += min(x / i, n);
    }
    return ans;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<u32>().unwrap();
    let k = it.next().unwrap().parse::<u32>().unwrap();
    writeln!(buf, "{}", lb(|x: u32| -> bool {
        return g(n, x) >= k;
    }, 1, 1_000_000_007)).unwrap();
}
