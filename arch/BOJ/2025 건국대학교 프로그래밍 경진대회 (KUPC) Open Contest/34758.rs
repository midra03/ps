use std::io::{stdin, Write, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let x = it.next().unwrap().parse::<i32>().unwrap();
    let y = it.next().unwrap().parse::<i32>().unwrap();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..n {
        let xi = it.next().unwrap().parse::<i32>().unwrap();
        let yi = it.next().unwrap().parse::<i32>().unwrap();
        if xi == x || yi == y {
            writeln!(buf, "0").unwrap();
        } else {
            writeln!(buf, "1").unwrap();
        }
    }
}

