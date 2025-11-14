use std::{io::{Read, Write, stdin}};

const MOD: i64 = 1e9 as i64 + 7;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let k = it.next().unwrap().parse::<usize>().unwrap();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut tree = vec![vec![false; 29]; 29];
    let mut dp = vec![vec![0; 27]; n];
    for _ in 0..k {
        let s = it.next().unwrap().as_bytes().to_vec();
        dp[0][(s[0] - 'a' as u8) as usize] = 1;
        tree[(s[0] - 'a' as u8) as usize][(s[1] - 'a' as u8) as usize] = true;
    }
    let mut ans = 0;
    for i in 1..n {
        for j in 0..('z' as u8 - 'a' as u8 + 1) as usize {
            for k in 0..('z' as u8 - 'a' as u8 + 1) as usize {
                if tree[k][j] {
                    dp[i][j] += dp[i - 1][k];
                    dp[i][j] %= MOD;
                }
            }
        }
    }
    for i in 1..n {
        for j in 0..('z' as u8 - 'a' as u8 + 1) as usize {
            ans += dp[i][j];
            ans %= MOD;
        }
    }
    writeln!(buf, "{}", ans).unwrap();
}