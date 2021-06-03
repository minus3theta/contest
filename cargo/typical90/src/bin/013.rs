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

fn dijkstra(adj: &[Vec<(usize, i64)>], start: usize) -> Vec<i64> {
    let mut dist = vec![1 << 60; adj.len()];
    dist[start] = 0;
    let mut que = BinaryHeap::new();
    use cmp::Reverse;
    que.push((Reverse(0), start));
    while let Some((d, v)) = que.pop() {
        let d = d.0;
        if d > dist[v] {
            continue;
        }
        for &(u, c) in &adj[v] {
            if d + c < dist[u] {
                dist[u] = d + c;
                que.push((Reverse(d + c), u));
            }
        }
    }
    dist
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1, i64); m],
    }
    let mut adj = vec![vec![]; n];
    for &(a, b, c) in &es {
        adj[a].push((b, c));
        adj[b].push((a, c));
    }
    let dist0 = dijkstra(&adj, 0);
    let dist1 = dijkstra(&adj, n - 1);

    for i in 0..n {
        println!("{}", dist0[i] + dist1[i]);
    }
}
