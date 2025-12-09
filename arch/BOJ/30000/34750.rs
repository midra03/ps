use std::io::{stdin, Write, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<i32>().unwrap();
    if n >= 1000000 {
        writeln!(buf, "{} {}", n / 5, n * 4 / 5).unwrap();
    } else if 500000 <= n && n < 1000000 {
        writeln!(buf, "{} {}", n * 3 / 20, n * 17 / 20).unwrap();
    } else if 100000 <= n && n < 500000 {
        writeln!(buf, "{} {}", n / 10, n * 9 / 10).unwrap();
    } else {
        writeln!(buf, "{} {}", n / 20, n * 19 / 20).unwrap();
    }
}
