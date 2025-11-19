use std::io::{stdin, Write, Read};

fn lb(a: &Vec<i32>, c: i32, f: fn(i32, i32) -> bool, le: usize, ri: usize) -> usize { // 닫힌 구간
    let mut lo = le as i32 - 1;
    let mut hi = ri as i32 + 1;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if !f(a[mid as usize], c) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    return hi as usize;
}

fn f(x: i32, c: i32) -> bool {
    return x >= c;
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
        let ai = it.next().unwrap().parse::<i32>().unwrap();
        a.push(ai);
    }
    a.sort();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..m {
        let x = it.next().unwrap().parse::<i32>().unwrap();
        let res = lb(&a, x, f, 0, n - 1);
        // println!("{}", res);
        if res < n && a[res] == x {
            writeln!(buf, "{}", 1).unwrap();
        } else {
            writeln!(buf, "{}", 0).unwrap();
        }
    }
}
