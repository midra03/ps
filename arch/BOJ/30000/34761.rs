use std::{collections::HashSet, io::{Read, Write, stdin}};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut a = vec![];
    let mut hs = HashSet::new();
    for _ in 0..n {
        let ai = it.next().unwrap().parse::<u32>().unwrap();
        a.push(ai);
        hs.insert(ai);
    }
    for i in 0..n {
        let bi = it.next().unwrap().parse::<u32>().unwrap();
        if bi != a[i] {
            writeln!(buf, "NO").unwrap();
            return;
        }
    }
    for _ in 0..n {
        let bi = it.next().unwrap().parse::<u32>().unwrap();
        if !hs.contains(&bi) {
            writeln!(buf, "NO").unwrap();
            return;
        }
    }
    writeln!(buf, "YES").unwrap();
}

