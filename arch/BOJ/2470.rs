use std::io::{stdin, Write, Read};

fn bs<F: Fn(i32) -> bool>(f: F, le: i32, ri: i32) -> usize {
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
    return hi as usize;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut a = vec![];
    for _ in 0..n {
        let ai = it.next().unwrap().parse::<i64>().unwrap();
        a.push(ai);
    }
    a.sort();
    let mut mi = 1_000_000_000_007;
    let mut ans = (0, 0);
    for i in 0..(n - 1) {
        let mut j = bs(|x: i32| -> bool {
            a[i] + a[x as usize] >= 0
        }, i as i32 + 1, n as i32 - 1);
        if j >= n {
            j -= 1;
        }
        if i64::abs(a[i] + a[j]) < mi {
            mi = i64::abs(a[i] + a[j]);
            ans = (a[i], a[j]);
        }
        if j - 1 > i && i64::abs(a[i] + a[j - 1]) < mi {
            mi = i64::abs(a[i] + a[j - 1]);
            ans = (a[i], a[j - 1]);
        }
    }
    if ans.0 > ans.1 {
        ans = (ans.1, ans.0);
    }
    writeln!(buf, "{} {}", ans.0, ans.1).unwrap();
}

