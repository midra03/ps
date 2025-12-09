use std::{io::{Read, Write, stdin}, str::{FromStr, SplitAsciiWhitespace}};

fn scan<T: FromStr>(it: &mut SplitAsciiWhitespace) -> Result<T, T::Err> {
    return it.next().unwrap().parse::<T>();
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let t = scan::<usize>(&mut it).unwrap();
    for _ in 0..t {
        let s: Vec<char> = it.next().unwrap().chars().collect();
        let n = s.len();
        let mut p = 0;
        let mut ok = true;
        for i in 0..n {
            if s[i] == '(' {
                p += 1;
            } else {
                p -= 1;
            }
            if p < 0 {
                ok = false;
                break;
            }
        }
        if ok && p == 0 {
            writeln!(buf, "YES").unwrap();
        } else {
            writeln!(buf, "NO").unwrap();
        }
    }
}
