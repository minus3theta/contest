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
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

const INF: i64 = 1 << 60;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        es: [(Usize1, Usize1, i64, i64); m],
    }
    let mut adj = vec![vec![]; n];
    for &(a, b, t, k) in &es {
        adj[a].push((b, t, k));
        adj[b].push((a, t, k));
    }
    let mut dist = vec![INF; n];
    dist[x] = 0;
    let mut que = BinaryHeap::new();
    que.push((Reverse(0i64), x));
    while let Some((d, v)) = que.pop() {
        let d = d.0;
        if d > dist[v] {
            continue;
        }
        for &(u, t, k) in &adj[v] {
            let dept = (d + k - 1) / k * k;
            let arr = dept + t;
            if arr < dist[u] {
                dist[u] = arr;
                que.push((Reverse(arr), u));
            }
        }
    }
    let ans = if dist[y] == INF { -1 } else { dist[y] };
    println!("{}", ans);
}
