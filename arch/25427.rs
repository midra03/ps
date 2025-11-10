use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let _ = it.next().unwrap().parse::<usize>().unwrap();
    let s = it.next().unwrap().as_bytes();
    let dksh = "DKSH".as_bytes();
    let mut dp = vec![[0i64; 5]; s.len() + 1];
    for i in 1..=s.len() {
        // 첫번째 문자가 D라면 계산 시작
        dp[i][1] = dp[i - 1][1];
        if s[i - 1] == dksh[0] {
            dp[i][1] += 1;
        }

        for j in 2..5 {
            dp[i][j] = dp[i - 1][j]; // 이전 결과는 그대로 전이 됨
            if s[i - 1] == dksh[j - 1] { // 만약 글자가 매칭된다면
                dp[i][j] += dp[i - 1][j - 1]; // 이전 글자까지의 경우의 수를 더함
            }
        }
    }
    // println!("{:?}", dp);
    println!("{}", dp.last().unwrap()[4]);
}