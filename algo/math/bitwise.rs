fn msb(n: i64) -> usize { // 비트 수를 알아내는 함수
    let mut ret = 0;
    let mut tmp = n;
    while tmp > 0{
        tmp /= 2;
        ret += 1;
    }
    return ret;
}

fn main() {
    println!("{}", msb(10));
}
