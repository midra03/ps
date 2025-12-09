use std::io::{stdin, Read};

fn sol(arr: &Vec<usize>) {
    let  n = arr.len();
    let mut loc = vec![0; n + 1];
    let mut ans = 0;
    for i in 0..n {
        loc[arr[i]] = i;
    }
    for i in 1..=n {
        ans += loc[i] + 1 - i;
        for j in i..=n {
            let t = loc[j];
            if i - 1 <= t && t < loc[i] {
                loc[j] += 1
            }
        }
        loc[i] = i - 1;
    }
    println!("{}", ans);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = it.next().unwrap().parse::<usize>().unwrap();
        let mut arr = vec![];
        for _ in 0..n {
            let a = it.next().unwrap().parse::<usize>().unwrap();
            arr.push(a);
        }
        sol(&arr);
    }
}