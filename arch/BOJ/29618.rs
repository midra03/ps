use std::{collections::BTreeSet, io::{Read, Write, stdin}, str::{FromStr, SplitAsciiWhitespace}};
use std::ops::Bound::Included;

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
    let q = scan::<usize>(&mut it).unwrap();
    let mut ans = vec![0; n + 1];
    let mut s = BTreeSet::new();
    for i in 1..=n {
        s.insert(i);
    }
    for _ in 0..q {
        let a = scan::<usize>(&mut it).unwrap();
        let b = scan::<usize>(&mut it).unwrap();
        let x = scan::<u32>(&mut it).unwrap();
        let mut rem = vec![];
        for &i in s.range((Included(&a), Included(&b))) {
            ans[i] = x;
            rem.push(i);
        }
        for i in rem {
            s.remove(&i);
        }
    }
    for i in 1..=n {
        write!(buf, "{} ", ans[i]).unwrap();
    }
    writeln!(buf, "").unwrap();
}
