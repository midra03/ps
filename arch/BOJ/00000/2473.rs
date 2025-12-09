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
    let mut ans = [0, 0, 0];
    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            let mut k = bs(|x: i32| -> bool {
                a[i] + a[j] + a[x as usize] >= 0
            }, j as i32 + 1, n as i32 - 1);
            if k >= n {
                k -= 1;
            }
            if i64::abs(a[i] + a[j] + a[k]) < mi {
                mi = i64::abs(a[i] + a[j] + a[k]);
                ans = [a[i], a[j], a[k]];
            }
            if k - 1 > j && i64::abs(a[i] + a[j] + a[k - 1]) < mi {
                mi = i64::abs(a[i] + a[j] + a[k - 1]);
                ans = [a[i], a[j], a[k - 1]];
            }
        }
    }
    ans.sort();
    writeln!(buf, "{} {} {}", ans[0], ans[1], ans[2]).unwrap();
}

