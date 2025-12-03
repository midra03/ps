use std::{cmp::min, io::{stdin, Read, Write}, str::{FromStr, SplitAsciiWhitespace}};

fn scan<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T {
    return it.next().unwrap().parse::<T>().ok().unwrap();
}

fn sol(n: usize, y: usize, r: usize) -> usize {
    return min(n, y / 2 + r);
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
        let y = scan::<usize>(&mut it);
        let r = scan::<usize>(&mut it);
        writeln!(buf, "{}", sol(n, y, r)).unwrap();
    }
}
