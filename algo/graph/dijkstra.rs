use std::collections::BinaryHeap;

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
    let ans = sol(3, &vec![vec![], vec![(2, 3), (3, 4)], vec![], vec![]], 1);
    println!("{:?}", ans);
}
