use std::{io::{Read, Write, stdin}, str::{FromStr, SplitAsciiWhitespace}};

fn scan<T: FromStr>(it: &mut SplitAsciiWhitespace) -> Result<T, T::Err> {
    return it.next().unwrap().parse::<T>();
}

fn in_range(x: i32, n: usize) -> bool {
    return 0 <= x && x < n as i32;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let r = scan::<usize>(&mut it).unwrap();
    let c = scan::<usize>(&mut it).unwrap();
    let h = scan::<usize>(&mut it).unwrap();
    let mut board: Vec<Vec<Vec<char>>> = vec![vec![]; h];
    for z in 0..h {
        for _ in 0..r {
            board[z].push(it.next().unwrap().chars().collect());
        }
    }
    let mut ans = vec![vec![vec![0; c]; r]; h];
    for z in 0..h {
        for y in 0..r {
            for x in 0..c {
                for dz in -1..=1 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dz == 0 && dy == 0 && dx == 0 {
                                continue;
                            }
                            let nz = z as i32 + dz;
                            let ny = y as i32 + dy;
                            let nx = x as i32 + dx;
                            if in_range(nz, h) && in_range(ny, r) && in_range(nx, c) && board[nz as usize][ny as usize][nx as usize] == '*' {
                                ans[z][y][x] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    for z in 0..h {
        for y in 0..r {
            for x in 0..c {
                if board[z][y][x] == '*' {
                    write!(buf, "*").unwrap();
                } else {
                    write!(buf, "{}", ans[z][y][x] % 10).unwrap();
                }
            }
            writeln!(buf).unwrap();
        }
    }
}
