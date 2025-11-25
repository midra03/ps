fn bs<F: Fn(i32) -> bool>(f: F, le: i32, ri: i32) -> usize { // 처음로 f(x)가 참인 x를 반환한다.
    let mut lo = le - 1;
    let mut hi = ri + 1;
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if f(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    return hi as usize;
}

fn main() {
    let a = vec![1, 2, 2, 3, 5, 7];

    println!("{}", bs(|x: i32| -> bool {
        a[x as usize] >= 2
    }, 0, a.len() as i32 - 1)); // lower bound

    println!("{}", bs(|x: i32| -> bool {
        a[x as usize] > 2
    }, 0, a.len() as i32 - 1)); // upper bound
}
