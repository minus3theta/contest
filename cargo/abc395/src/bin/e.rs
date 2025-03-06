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
        n: usize,
        m: usize,
        x: i64,
        es: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![]; n * 2];
    for &(a, b) in &es {
        adj[a].push((b, 1));
        adj[b + n].push((a + n, 1));
    }
    for i in 0..n {
        adj[i].push((i + n, x));
        adj[i + n].push((i, x));
    }
    let mut dist = vec![1i64 << 60; n * 2];
    dist[0] = 0;
    let mut que = std::collections::BinaryHeap::new();
    que.push((cmp::Reverse(0), 0));
    while let Some((cmp::Reverse(d), i)) = que.pop() {
        if dist[i] < d {
            continue;
        }
        for &(j, c) in &adj[i] {
            let d = d + c;
            if d < dist[j] {
                dist[j] = d;
                que.push((cmp::Reverse(d), j));
            }
        }
    }
    println!("{}", dist[n - 1].min(dist[2 * n - 1]));
}
