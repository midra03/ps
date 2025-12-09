use std::io::{stdin, Write, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = it.next().unwrap().parse::<usize>().unwrap();
        if n % 2 == 0 {
            if n == 2 {
                writeln!(buf, "2 - 1 - 1").unwrap();
            } else {
                let k = (n - 2) / 2;
                writeln!(buf, "{} - {} + 1", k + 1, k).unwrap();
            }
        } else {
            if n == 1 {
                writeln!(buf, "1 + 1 - 1").unwrap();
            } else if n == 3 {
                writeln!(buf, "4 - 3 - 2").unwrap();
            } else if n == 5 {
                writeln!(buf, "5 - 4 - 2").unwrap();
            } else {
                let k = (n - 5) / 2;
                writeln!(buf, "{} - {} + 2", k + 1, k).unwrap();
            }
        }
    }
}