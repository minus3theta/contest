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

fn score(diff: i64) -> i64 {
    if diff >= 0 {
        0
    } else {
        diff
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i64; n],
        e: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![]; n];
    for &(u, v) in &e {
        let c = h[u] - h[v];
        adj[u].push((v, score(c)));
        adj[v].push((u, score(-c)));
    }
    let mut gain = vec![std::i64::MIN; n];
    gain[0] = h[0];
    let mut que = BinaryHeap::new();
    que.push((h[0], 0));
    while let Some((g, i)) = que.pop() {
        for &(v, s) in &adj[i] {
            if gain[v] >= g + s {
                continue;
            }
            gain[v] = g + s;
            que.push((gain[v], v));
        }
    }

    let ans = gain.iter().zip(&h).map(|(&g, &x)| g - x).max().unwrap();

    println!("{}", ans);
}
