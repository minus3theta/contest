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
use std::cmp::{self, Reverse};
#[allow(unused_imports)]
use std::collections::*;

const INF: i64 = 1 << 60;

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
    }
    let mut dist = vec![vec![INF; n]; n];
    for src in 0..n {
        let mut que = BinaryHeap::new();
        que.push((Reverse(0), src));
        while let Some((d, v)) = que.pop() {
            let d = d.0;
            if d > dist[src][v] {
                continue;
            }
            for &(u, c) in &adj[v] {
                if d + c < dist[src][u] {
                    dist[src][u] = d + c;
                    que.push((Reverse(d + c), u));
                }
            }
        }
    }
    for i in 0..n {
        let d = dist[i][i];
        println!("{}", if d == INF { -1 } else { d });
    }
}
