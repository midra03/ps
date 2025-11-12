use std::io::{stdin, Read};

fn sol(s: &Vec<char>, lo: i32, hi: i32) -> Vec<String> {
    if hi < lo {
        return vec![];
    }
    if lo == hi {
        return vec![String::from(s[lo as usize])];
    }
    let mut ret = vec![];
    
    // 기준 문자를 찾는다. 
    let mut c = ('Z' as u8 + 1) as char;
    let mut pos = -1;
    for i in lo..=hi {
        if s[i as usize] < c {
            pos = i;
            c = s[i as usize];
        }
    }
    ret.push(String::from(c));
    let before = sol(s, lo, pos - 1);
    let after = sol(s, pos + 1, hi);
    for a in after {
        ret.push(format!("{}{}", c, a));
    }
    let last = ret .last().unwrap().clone();
    for b in  before {
        ret.push(format!("{}{}", b, last));
    }
    // println!("{} {}: {:?}", lo, hi, ret);
    return ret;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let s: Vec<char> = it.next().unwrap().chars().collect();
    let ans = sol(&s, 0, s.len() as i32 -1);
    for a in ans {
        println!("{}", a);
    }
}