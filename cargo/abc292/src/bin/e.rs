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

fn reachable(v: usize, adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    let mut visited = vec![false; n];
    dfs(v, adj, &mut visited);
    visited.iter().filter(|&&b| b).count() - 1
}

fn dfs(v: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
    visited[v] = true;
    for &u in &adj[v] {
        if visited[u] {
            continue;
        }
        dfs(u, adj, visited);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![]; n];
    for (u, v) in es {
        adj[u].push(v);
    }
    let ans = (0..n).map(|v| reachable(v, &adj)).sum::<usize>() - m;

    println!("{}", ans);
}
