use std::io::{stdin, Write, Read};

struct UF {
    p: Vec<usize>,
    rank: Vec<u32>,
}

impl UF {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n + 1];
        for i in 0..=n {
            p[i] = i
        }
        UF {
            p: p,
            rank: vec![0; n + 1]
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.p[i] == i {
            return i;
        } else {
            self.p[i] = self.find(self.p[i]);
            return self.p[i];
        }
    }

    fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(self.p[i]) == self.find(self.p[j])
    }

    fn union(&mut self, i: usize, j: usize) {
        if !self.same(i, j) {
            let u = self.find(i);
            let v = self.find(j);
            if self.rank[u] > self.rank[v] {
                self.p[v] = u;
            } else {
                self.p[u] = v;
                if self.rank[u] == self.rank[v] {
                    self.rank[v] += 1;
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let stdout = std::io::stdout();
    let mut buf = std::io::BufWriter::new(stdout.lock());
    let n = it.next().unwrap().parse::<usize>().unwrap();
    let mut uf = UF::new(n);
    let m = it.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..m {
        let q = it.next().unwrap().parse::<usize>().unwrap();
        let a = it.next().unwrap().parse::<usize>().unwrap();
        let b = it.next().unwrap().parse::<usize>().unwrap();
        if q == 0 {
            uf.union(a, b);
        } else {
            if uf.same(a, b) {
                writeln!(buf, "YES").unwrap();
            } else {
                writeln!(buf, "NO").unwrap();
            }
        }
    }
}

