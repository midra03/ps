use std::io::{stdin, Write, Read};

fn msb(n: i64) -> usize {
    let mut ret = 0;
    let mut tmp = n;
    while tmp > 0{
        tmp /= 2;
        ret += 1;
    }
    return ret;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let k = it.next().unwrap().parse::<i64>().unwrap();
    if k == 0 {
        writeln!(buf, "0").unwrap();
        return;
    }
    if k % 2 == 0 {
        writeln!(buf, "-1").unwrap();
        return;
    }
    writeln!(buf, "{}", msb(i64::abs(k))).unwrap();
}
