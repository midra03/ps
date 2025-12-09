use std::io::{stdin, Write, Read};

const STAR: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
];

fn build(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::from("*")];
    }
    let bef = build(n - 1);
    let size = bef.len();
    let mut ret = vec![String::new(); size * 5];
    for i in 0..5 {
        for j in 0..5 {
            if STAR[i][j] == 0 {
                for y in 0..size {
                    let mut tmp = String::new();
                    for _ in 0..size {
                        tmp.push(' ');
                    }
                    ret[i * size + y].push_str(&tmp);
                }
            } else {
                for y in 0..size {
                    ret[i * size + y].push_str(&bef[y]);
                }
            }
        }
    }
    return ret;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<u8>().unwrap();
    let ans = build(n);
    for a in ans {
        writeln!(buf, "{}", a).unwrap();
    }
}

