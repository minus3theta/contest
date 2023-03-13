#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [u8; n],
            es: [(Usize1, Usize1); m],
        }

        println!("{}", solve(&c, &es));
    }
}

fn solve(c: &[u8], es: &[(usize, usize)]) -> i64 {
    let n = c.len();
    let mut adj = vec![vec![]; n];
    for &(u, v) in es {
        adj[u].push(v);
        adj[v].push(u);
    }

    let start = (0, n - 1);
    let mut dist = HashMap::new();
    dist.insert(start, 0);

    let mut que = VecDeque::new();
    que.push_back(start);
    while let Some((a, t)) = que.pop_front() {
        let d = dist[&(a, t)];
        for (&na, &nt) in adj[a].iter().cartesian_product(&adj[t]) {
            if c[na] == c[nt] || dist.contains_key(&(na, nt)) {
                continue;
            }
            dist.insert((na, nt), d + 1);
            que.push_back((na, nt));
        }
    }

    *dist.get(&(n - 1, 0)).unwrap_or(&-1)
}
