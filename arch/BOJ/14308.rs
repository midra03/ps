use std::{cmp::min, io::{Read, Write, stdin}};

fn fit(b: &Vec<Vec<bool>>, y: usize, x: usize, s: usize) -> bool {
    for i in y..(y + s) {
        for j in x..(x + s) {
            if b[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn sol(b: &Vec<Vec<bool>>) -> usize {
    let mut ans = 0;
    let n = b.len();
    let m = b[0].len();
    for k in 1..=min(n, m) {
        for i in 0..(n - k + 1) {
            for j in 0..(m - k + 1) {
                if fit(b, i, j, k) {
                    ans += 1;
                }
            }
        }
    }
    return ans;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for i in 1..=t {
        let n = it.next().unwrap().parse::<usize>().unwrap();
        let m = it.next().unwrap().parse::<usize>().unwrap();
        let mut b = vec![vec![false; m]; n];
        let k = it.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..k {
            let r = it.next().unwrap().parse::<usize>().unwrap();
            let c = it.next().unwrap().parse::<usize>().unwrap();
            b[r][c] = true;
        }
        writeln!(buf, "Case #{}: {}", i, sol(&b)).unwrap();
    }
}

