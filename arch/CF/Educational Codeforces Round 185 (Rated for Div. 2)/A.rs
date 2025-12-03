use std::{cmp::max, io::{stdin, Read, Write}, str::{FromStr, SplitAsciiWhitespace}};

fn scan<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T {
    return it.next().unwrap().parse::<T>().ok().unwrap();
}

fn sol(n: usize) -> i32 {
    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = (i * n + j + 1) as i32;
        }
    }
    let mut ret = 0;
    for i in 0..n as i32 {
        for j in 0..n as i32 {
            let mut tmp = b[i as usize][j as usize];
            if i - 1 >= 0 {
                tmp += b[i as usize - 1][j as usize];
            }
            if i + 1 < n as i32 {
                tmp += b[i as usize + 1][j as usize];
            }
            if j - 1 >= 0 {
                tmp += b[i as usize][j as usize - 1];
            }
            if j + 1 < n as i32 {
                tmp += b[i as usize][j as usize + 1];
            }
            ret = max(ret, tmp);
        }
    }
    return ret;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let t = scan::<usize>(&mut it);
    for _ in 0..t {
        let n = scan::<usize>(&mut it);
        writeln!(buf, "{}", sol(n)).unwrap();
    }
}

