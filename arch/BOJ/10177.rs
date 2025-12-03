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
        let mut b = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                b[i][j] = it.next().unwrap().parse::<u32>().unwrap();
            }
        }
        let mut ok = true;
        let mut sum = 0;
        for i in 0..n {
            sum += b[0][i];
        }
        for i in 0..n {
            let mut tmp = 0;
            for j in 0..n {
                tmp += b[i][j];
            }
            if tmp != sum {
                ok = false;
            }
        }
        for i in 0..n {
            let mut tmp = 0;
            for j in 0..n {
                tmp += b[j][i];
            }
            if tmp != sum {
                ok = false;
            }
        }
        let mut d1 = 0;
        for i in 0..n {
            d1 += b[i][i];
        }
        if d1 != sum {
            ok = false;
        }
        let mut d2 = 0;
        for i in 0..n {
            d2 += b[i][n - i - 1];
        }
        if d2 != sum {
            ok = false;
        }
        if ok {
            writeln!(buf, "Magic square of size {}", n).unwrap();
        } else {
            writeln!(buf, "Not a magic square").unwrap();
        }
    }
}

