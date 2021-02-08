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

const INF: i64 = 1 << 60;

fn dist_from(src: usize, adj: &[Vec<usize>]) -> Vec<i64> {
    let mut dist = vec![INF; adj.len()];
    dist[src] = 0;
    let mut que = VecDeque::new();
    que.push_back(src);
    while let Some(v) = que.pop_front() {
        let d = dist[v];
        for &u in &adj[v] {
            if d + 1 < dist[u] {
                dist[u] = d + 1;
                que.push_back(u);
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
        ps: [(Usize1, Usize1); m],
        k: usize,
        c: [Usize1; k],
    }
    let mut adj = vec![vec![]; n];
    for &(a, b) in &ps {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut dist_c = vec![vec![INF; k]; k];
    for src in 0..k {
        let dist = dist_from(c[src], &adj);
        for dst in 0..k {
            dist_c[src][dst] = dist[c[dst]];
        }
    }
    let mut dp = vec![vec![INF; k]; 1 << k];
    for set in 0..1 << k {
        for dst in 0..k {
            if set & (1 << dst) != 0 {
                continue;
            }
            if set == 0 {
                dp[1 << dst][dst] = 1;
                continue;
            }
            let mut d = INF;
            for src in 0..k {
                if set & (1 << src) == 0 {
                    continue;
                }
                d = cmp::min(d, dp[set][src] + dist_c[src][dst]);
            }
            dp[set | (1 << dst)][dst] = d;
        }
    }
    let ans = *dp[(1 << k) - 1].iter().min().unwrap();
    println!("{}", if ans == INF { -1 } else { ans });
}
