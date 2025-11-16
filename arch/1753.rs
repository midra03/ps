use std::{collections::BinaryHeap, io::{Read, Write, stdin}};

const INF: i32 = 1e9 as i32 + 9;

fn sol(n: usize, graph: &Vec<Vec<(usize, i32)>>, s: usize) -> Vec<i32> {
    let mut dist = vec![INF; n + 1];
    let mut pq = BinaryHeap::new();
    dist[s] = 0;
    pq.push((0, s));
    while !pq.is_empty() {
        let peek = pq.pop().unwrap();
        let d = -peek.0;
        let u = peek.1;
        if d > dist[u] {
            continue;
        }
        for i in 0..graph[u].len() {
            let v = graph[u][i].0;
            let w = graph[u][i].1;
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                pq.push((-dist[v], v));
            }
        }
    }
    return dist;
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let v = it.next().unwrap().parse::<usize>().unwrap();
    let e = it.next().unwrap().parse::<usize>().unwrap();
    let k = it.next().unwrap().parse::<usize>().unwrap();
    let mut graph = vec![vec![]; v + 1];
    for _ in 0..e {
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let b = it.next().unwrap().parse::<usize>().unwrap();
        let c = it.next().unwrap().parse::<i32>().unwrap();
        graph[a].push((b, c));
    }
    let ans = sol(v, &graph, k);
    for i in 1..=v {
        if ans[i] == INF {
            writeln!(buf, "INF").unwrap();
        } else {
            writeln!(buf, "{}", ans[i]).unwrap();
        }
    }
}
