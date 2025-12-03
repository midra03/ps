use std::io::{stdin, Write, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut x = vec![];
    let mut y = vec![];
    for _ in 0..n {
        let xi = it.next().unwrap().parse::<i64>().unwrap();
        let yi = it.next().unwrap().parse::<i64>().unwrap();
        x.push(xi);
        y.push(yi);
    }
    x.sort();
    y.sort();
    let ans_x = x[n / 2];
    let ans_y = y[n / 2];
    let mut ans = 0;
    for i in 0..n {
        ans += i64::abs(ans_x - x[i]) + i64::abs(ans_y - y[i]);
    }
    writeln!(buf, "{}", ans).unwrap();
}

