use std::{collections::BinaryHeap, io::{Read, Write, stdin}};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let k = it.next().unwrap().parse::<i32>().unwrap();
    let mut pq = BinaryHeap::new();
    for _ in 0..m {
        pq.push(0);
    }
    for _ in 0..n {
        let t = it.next().unwrap().parse::<i32>().unwrap();
        let last = pq.pop().unwrap();
        pq.push(last - t);
    }
    let rhs = -pq.peek().unwrap();
    if rhs <= k {
        writeln!(buf, "WAIT").unwrap();
    } else {
        writeln!(buf, "GO").unwrap();
    }
}