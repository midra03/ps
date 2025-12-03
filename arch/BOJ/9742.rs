use std::io::{stdin, Write, Read};

fn fact(n: usize) -> usize {
    let mut ret = 1;
    for i in 2..=n {
        ret *= i;
    }
    return ret;
}

/*
l: 현재 남은 길이
k: 몇번째인지... (zero index임에 주의)
s: 상태 (0: 안쓴거, 1: 쓴거) (이 역시 zero index)
*/
fn sol(ret: &mut Vec<usize>, n: usize, l: usize, k: usize, s: i32) {
    // println!("{:?} {} {} {}", ret, l, k, s);
    if l == 0 {
        return;
    }
    let f = fact(l - 1);
    let mut unused = vec![];
    for i in 0..n {
        if (s & (1 << i)) == 0 {
            unused.push(i);
        }
    }
    let i = unused[k / f];
    ret.push(i);
    sol(ret, n, l - 1, k % f, s | (1 << i));
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    loop {
        let mut ret = vec![];
        let s: Vec<char> = match it.next() {
            Some(x) => {
                write!(buf, "{} ", x).unwrap();
                x.chars().collect()
            },
            _ => break
        };
        let mut k = it.next().unwrap().parse::<usize>().unwrap();
        write!(buf, "{} = ", k).unwrap();
        k -= 1;
        if k > (fact(s.len()) - 1) {
            writeln!(buf, "No permutation").unwrap();
        } else {
            sol(&mut ret, s.len(), s.len(), k, 0);
            for i in 0..s.len() {
                write!(buf, "{}", s[ret[i]]).unwrap();
            }
            writeln!(buf).unwrap();
        }
    }
}