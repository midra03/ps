use std::io::{stdin, Read};

const MOD: i64 = 10i64.pow(9) + 7;

fn count(comb: &Vec<Vec<i64>>, k: usize) -> i64 {
    match k {
        1 => 1,
        2 => 2,
        _ => {
            let mut ans = 0;
            for i in 0..=(k - 1) { // i는 최대값 이전에 오는 값의 수
                ans += comb[k - 1][i] % MOD;
                ans %= MOD;
            }
            ans
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let k = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..n {
        let _ = it.next().unwrap().parse::<usize>().unwrap();
    }
    let mut comb = vec![vec![1], vec![1, 1]];
    for i in 2..=2007 {
        let mut tmp = vec![1];
        for j in 0..(i - 1) {
            tmp.push((comb[i - 1][j] + comb[i - 1][j + 1]) % MOD);
        }
        tmp.push(1);
        comb.push(tmp);
    }
    println!("{}", (comb[n][k] * count(&comb, k)) % MOD);
}