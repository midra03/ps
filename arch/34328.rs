use std::io::{Read, Write, stdin};

const MAX: usize = 5e5 as usize + 7;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut ans = vec![None; MAX];
    ans[1] = Some(vec![1]);
    let mut i = 2;
    while ((1 << i) - 2) < MAX {
        let mut tmp = vec![];
        for i in 1..=((1 << i) - 2) {
            tmp.push(i);
        }
        ans[(1 << i) - 2] = Some(tmp);
        i += 1;
    }
    match &ans[n] {
        Some(x) => {
            for i in x {
                write!(buf, "{} ", i).unwrap();
            }
            writeln!(buf).unwrap()
        },
        _ => {
            writeln!(buf, "-1").unwrap();
        }
    }

}
