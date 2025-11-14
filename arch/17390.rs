use std::io::{stdin, Write, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let mut a = vec![];
    for _ in 0..n {
        let ai = it.next().unwrap().parse::<i64>().unwrap();
        a.push(ai);
    }
    a.sort();
    let mut s = vec![0];
    for i in 0..n {
        s.push(s.last().unwrap() + a[i]);
    }
    for _ in 0..m {
        let l = it.next().unwrap().parse::<usize>().unwrap();
        let r = it.next().unwrap().parse::<usize>().unwrap();
        writeln!(buf, "{}", s[r] - s[l - 1]).unwrap();
    }
}