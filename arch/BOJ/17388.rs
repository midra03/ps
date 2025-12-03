use std::io::{stdin, Write, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let s = it.next().unwrap().parse::<usize>().unwrap();
    let k = it.next().unwrap().parse::<usize>().unwrap();
    let h = it.next().unwrap().parse::<usize>().unwrap();
    if (s + k + h) >= 100 {
        writeln!(buf, "OK").unwrap();
    } else {
        if s < k && s < h {
            println!("Soongsil");
        }
        else if k < s && k < h {
            println!("Korea");
        } else {
            println!("Hanyang");
        }
    }
}
