use std::{collections::BTreeSet, io::{stdin, Read, Write}, str::{FromStr, SplitAsciiWhitespace}};

fn scan<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T {
    return it.next().unwrap().parse::<T>().ok().unwrap();
}

fn sol(n: usize, c: &Vec<u32>) -> usize {
    let mut s = BTreeSet::new();
    for i in 0..n {
        s.insert(c[i]);
    }
    let mut vis = [false; 1009];
    let mut cur = s.len();
    while !vis[cur] {
        vis[cur] = true;
        s.insert(cur as u32);
        cur = s.len();
    }
    return cur;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    for _ in 0..scan::<usize>(&mut it) {
        let n = scan::<usize>(&mut it);
        let mut c = vec![];
        for _ in 0..n {
            c.push(scan::<u32>(&mut it));
        }
        writeln!(buf, "{}", sol(n, &c)).unwrap();
    }
}
