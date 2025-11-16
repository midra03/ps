use std::io::{stdin, Write, Read};

fn sol(s: (i32, i32), e: (i32, i32), circles: &Vec<(i32, i32, i32)>) -> i32 {
    let mut ans = 0;
    for cir in circles {
        if ((s.0 - cir.0) * (s.0 - cir.0) + (s.1 - cir.1) * (s.1 - cir.1) < cir.2 * cir.2
            || (e.0 - cir.0) * (e.0 - cir.0) + (e.1 - cir.1) * (e.1 - cir.1) < cir.2 * cir.2)
            && !((s.0 - cir.0) * (s.0 - cir.0) + (s.1 - cir.1) * (s.1 - cir.1) < cir.2 * cir.2
            && (e.0 - cir.0) * (e.0 - cir.0) + (e.1 - cir.1) * (e.1 - cir.1) < cir.2 * cir.2) {
                ans += 1;
            }
    }
    return ans;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let t = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let sx = it.next().unwrap().parse::<i32>().unwrap();
        let sy = it.next().unwrap().parse::<i32>().unwrap();
        let ex = it.next().unwrap().parse::<i32>().unwrap();
        let ey = it.next().unwrap().parse::<i32>().unwrap();
        let mut circles = vec![];
        let n = it.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..n {
            let cx = it.next().unwrap().parse::<i32>().unwrap();
            let cy = it.next().unwrap().parse::<i32>().unwrap();
            let r = it.next().unwrap().parse::<i32>().unwrap();
            circles.push((cx, cy, r));
        }
        writeln!(buf, "{}", sol((sx, sy), (ex, ey), &circles)).unwrap();
    }
}
