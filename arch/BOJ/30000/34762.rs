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
    let n = scan::<usize>(&mut it).unwrap();
    let k = scan::<usize>(&mut it).unwrap();
    let m = scan::<usize>(&mut it).unwrap();
    let mut a = vec![];
    for _ in 0..m {
        a.push(scan::<usize>(&mut it).unwrap());
    }
    a.push(n + 1);
    let mut d = a[0] + k + 1;
    for i in 1..=m {
        if (a[i] - a[i - 1]) == 1 {
            d = a[i] + k + 1;
            continue;
        }
        if a[i] <= d {
            writeln!(buf, "NO").unwrap();
            return;
        }
        d = a[i];
    }
    writeln!(buf, "YES").unwrap();
}
