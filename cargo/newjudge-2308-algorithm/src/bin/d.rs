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
        n1: usize,
        n2: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![]; n1 + n2];
    for &(u, v) in &es {
        adj[u].push(v);
        adj[v].push(u);
    }

    println!("{}", bfs(&adj, 0) + bfs(&adj, n1 + n2 - 1) + 1);
}

fn bfs(adj: &[Vec<usize>], s: usize) -> i64 {
    let mut dist = vec![i64::MAX; adj.len()];
    let mut que = VecDeque::new();
    que.push_back(s);
    dist[s] = 0;
    let mut ret = 0;
    while let Some(v) = que.pop_front() {
        ret = ret.max(dist[v]);
        for &u in &adj[v] {
            let d = dist[v] + 1;
            if dist[u] <= d {
                continue;
            }
            dist[u] = d;
            que.push_back(u);
        }
    }
    ret
}
