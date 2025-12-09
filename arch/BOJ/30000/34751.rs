use std::io::{stdin, Write, Read};

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
    let mut i = 0;
    let mut seg = vec![];
    while i < n {
        let mut j = i + 1;
        while j < n {
            if a[i] == a[j] {
                j += 1;
            } else {
                break;
            }
        }
        seg.push((a[i], (i, j - 1)));
        i = j;
    }
    let mut ans = 0i64;
    let mut seg_i = 0;
    for pos in 0..n {
        if pos > seg[seg_i].1.1 {
            seg_i += 1;
        }
        if seg[seg_i].0 == 0 {
            ans += seg[seg_i].1.1 as i64 - pos as i64 + 1;
        }
        if seg_i < seg.len() - 1 {
            ans += 2 * (n as i64 - seg[seg_i + 1].1.0 as i64);
        }
    }
    writeln!(buf, "{}", ans).unwrap();
}
