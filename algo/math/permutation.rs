fn permutation(s: &Vec<char>, used: i32) -> Vec<String> {
    let mut ret = vec![];
    for i in 0..s.len() {
        if (used & (1 << i)) != 0 {
            continue;
        }
        let nused = used | (1 << i);
        let tmp = permutation(s, nused);
        for t in tmp {
            ret.push(format!("{}{}", s[i], t));
        }
    }
    if ret.is_empty() {
        ret.push("".to_string());
    }
    return ret;
}

fn main() {
    let s = "abcd".chars().collect();
    println!("{:?}", permutation(&s, 0));
}