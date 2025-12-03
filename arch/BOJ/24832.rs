use std::io::{Read, Write, stdin};

fn pal(s: &String) -> bool {
    let l = s.len();
    let tmp = s.as_bytes();
    for i in 0..(l / 2) {
        if tmp[i] != tmp[l - i - 1] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let mut arr = vec![];
    for _ in 0..n {
        let s = it.next().unwrap();
        arr.push(s.chars().collect::<String>());
    }
    let mut ans = String::new();
    let mut odd = false;
    for i in 0..n {
        for j in (i + 1)..n {
            if arr[i] == arr[j].chars().rev().collect::<String>() {
                ans.push_str(&arr[i]);
            }
        }
    }
    for s in arr {
        if pal(&s) {
            odd = true;
            ans.push_str(s.as_str());
            break;
        }
    }
    if odd {
        for _ in 0..(m / 2) {
            ans.pop();
        }
        if m % 2 == 0 {
            ans.push_str(&ans.chars().rev().collect::<String>());
        } else {
            let lc = ans.pop().unwrap();
            let rev = ans.chars().rev().collect::<String>();
            ans.push(lc);
            ans.push_str(&rev);
        }
    } else {
        ans.push_str(&ans.chars().rev().collect::<String>());
    }
    writeln!(buf, "{}", ans.len()).unwrap();
    writeln!(buf, "{}", ans).unwrap();
}


