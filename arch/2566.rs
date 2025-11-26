use std::{collections::BinaryHeap, io::{Read, Write, stdin}, str::{FromStr, SplitAsciiWhitespace}};

fn scan<T: FromStr>(it: &mut SplitAsciiWhitespace) -> Result<T, T::Err> {
    return it.next().unwrap().parse::<T>();
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let mut pq = BinaryHeap::new();
    for i in 0..9 {
        for j in 0..9 {
            let x = scan::<u32>(&mut it).unwrap();
            pq.push((x, i, j));
        }
    }
    let ans = pq.pop().unwrap();
    writeln!(buf, "{}\n{} {}", ans.0, ans.1 + 1, ans.2 + 1).unwrap();
}
