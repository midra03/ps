use std::{collections::VecDeque, io::{Read, stdin}};

fn dfs(tree: &Vec<Vec<usize>>, vis: &mut Vec<bool>, u: usize) {
    print!("{} ", u);
    vis[u] = true;
    for i in 0..tree[u].len() {
        let v = tree[u][i];
        if vis[v] {
            continue;
        }
        dfs(tree, vis, v);
    }
}

fn bfs(tree: &Vec<Vec<usize>>, vis: &mut Vec<bool>, s: usize) {
    let mut q = VecDeque::new();
    q.push_back(s);
    vis[s] = true;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        print!("{} ", u);
        for i in 0..tree[u].len() {
            let v = tree[u][i];
            if vis[v] {
                continue;
            }
            vis[v] = true;
            q.push_back(v);
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let m = it.next().unwrap().parse::<usize>().unwrap();
    let v = it.next().unwrap().parse::<usize>().unwrap();
    let mut tree = vec![vec![]; n + 1];
    for _ in 0..m {
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let  b = it.next().unwrap().parse::<usize>().unwrap();
        tree[a].push(b);
        tree[b].push(a);
    }
    for i in 1..=n {
        tree[i].sort();
    }
    dfs(&tree, &mut vec![false; n + 1], v);
    println!();
    bfs(&tree, &mut vec![false; n + 1], v);
    println!()
}