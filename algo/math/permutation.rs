fn permutation(s: &Vec<char>, used: Vec<bool>) -> Vec<String> {
    let mut ret = vec![];
    for i in 0..used.len() {
        if used[i] {
            continue;
        }
        let mut nused = used.clone();
        nused[i] = true;
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
    println!("{:?}", permutation(&s, vec![false; 4]));
}