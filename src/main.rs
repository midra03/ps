use std::{io::{Read, Write, stdin}, str::{FromStr, SplitAsciiWhitespace}};

fn scan<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T {
    return it.next().unwrap().parse::<T>().ok().unwrap();
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
}
