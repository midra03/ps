use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let c = it.next().unwrap().parse::<char>().unwrap();
    println!("{}", match c {
        'M' => "MatKor",
        'W' => "WiCys",
        'C' => "CyKor",
        'A' => "AlKor",
        '$' => "$clear",
        _ => "",
    });
}