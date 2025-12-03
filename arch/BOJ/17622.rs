use std::io::{Read, Write, stdin};

fn add(n: i32, pos: (i32, i32), d: Option<(i32, i32)>) -> Option<(i32, i32)> {
    if d.is_none() {
        return None;
    }
    let du = d.unwrap();
    if pos.0 + du.0 < 0 || pos.0 + du.0 >= n {
        return None;
    }
    if pos.1 + du.1 < 0 || pos.1 + du.1 >= n {
        return None;
    }
    return Some((pos.0 + du.0, pos.1 + du.1));
}

fn go(n: i32, t: usize, u: (i32, i32), dir: i32) -> (Option<(i32, i32)>, i32) {
    let d = [
        [None, None, Some((1, 0)), Some((0, 1))],
        [Some((1, 0)), None, None, Some((0, -1))],
        [None, Some((0, 1)), Some((-1, 0)), None],
        [Some((-1, 0)), Some((0, -1)), None, None],
        [None, Some((1, 0)), None, Some((-1, 0))],
        [Some((0, 1)), None, Some((0, -1)), None]
    ];
    let rot = [
        [-1, -1, 1, 0],
        [1, -1, -1, 2],
        [-1, 0, 3, -1],
        [3, 2, -1, -1],
        [-1, 1, -1, 3],
        [0, -1, 2, -1],
    ];
    return (add(n, u, d[t][dir as usize]), rot[t][dir as usize]);
}

fn simul(board: &Vec<Vec<usize>>) -> i32 {
    let n = board.len() as i32;
    let mut pos = Some((0, 0));
    let mut dir = 0;
    let mut ans = 0;
    while !pos.is_none() {
        let posu = pos.unwrap();
        let next = go(n, board[posu.0 as usize][posu.1 as usize], posu, dir);
        pos = next.0;
        dir = next.1;
        ans += 1;
        if posu == (n - 1, n - 1) && dir == 0 {
            return ans;
        }
    }
    return -1;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let k = it.next().unwrap().parse::<usize>().unwrap();
    let mut board = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            board[i][j] = it.next().unwrap().parse::<usize>().unwrap();
        }
    }
    if k == 0 {
        writeln!(buf, "{}", simul(&board)).unwrap();
        return;
    }
    let mut ans = -1;
    for i in 0..n {
        for j in 0..n {
            for l in 0..6 {
                if board[i][j] == l {
                    continue;
                }
                let mut nboard = board.clone();
                nboard[i][j] = l;
                let tmp = simul(&nboard);
                if (ans == -1 && tmp > 0) || (ans > 0 && tmp > 0 && tmp < ans) {
                    ans = tmp;
                }
            }
        }
    }
    writeln!(buf, "{}", ans).unwrap();
}
