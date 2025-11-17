use std::{cmp::{max, min}, io::{Read, Write, stdin}};

fn msb(n: i32) -> i32 {
    let mut ret = 0;
    let mut tmp = n;
    while tmp > 0{
        tmp /= 2;
        ret += 1;
    }
    return ret;
}

fn inv(n: i32, y: i32) -> i32 {
    // println!("{} {}", n, y);
    let mut ret = 0;
    for i in 0..30 {
        if (y & (1 << i)) != 0 {
            if (n & (1 << i)) != 0 {
                ret |= 1 << i;
            } else {
                return -1;
            }
        }
    }
    return ret;
}

fn sol(a: i32, b: i32) -> Option<Vec<(char, i32)>> {
    let ma = max(a, b);
    let mut mi = min(a, b);
    if (ma - mi) % 2 == 1 {
        return None;
    }
    let tmp = inv(ma, (ma - mi) / 2);
    if tmp != -1 {
        if a > b {
            return Some(vec![('A', tmp)]);
        }
        return Some(vec![('B', tmp)]);
    }
    let t = (1 << msb(ma)) - 1;
    let d = t - ma;
    mi ^= d;
    let x = inv(t, (t - mi) / 2);
    if a > b {
        return Some(vec![('B', t - ma), ('A', x)]);
    } else {
        return Some(vec![('A', t - ma), ('B', x)]);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let a = it.next().unwrap().parse::<i32>().unwrap();
        let b = it.next().unwrap().parse::<i32>().unwrap();
        let ans = sol(a, b);
        match ans {
            Some(x) => {
                writeln!(buf, "{}", x.len()).unwrap();
                for xx in x {
                    writeln!(buf, "{} {}", xx.0, xx.1).unwrap();
                }
            },
            _ => {
                writeln!(buf, "-1").unwrap();
            }
        }
    }
}
