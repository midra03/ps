use std::io::{stdin, Write, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let v = it.next().unwrap().parse::<usize>().unwrap();
        let e = it.next().unwrap().parse::<usize>().unwrap();
        writeln!(buf, "{}", e + 2 - v).unwrap();
    }
}
