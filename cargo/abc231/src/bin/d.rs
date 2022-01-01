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

fn dfs(adj: &[Vec<usize>], visited: &mut [bool], v: usize, prev: Option<usize>) -> bool {
    visited[v] = true;
    for &u in &adj[v] {
        if prev == Some(u) {
            continue;
        }
        if visited[u] {
            return true;
        }
        if dfs(adj, visited, u, Some(v)) {
            return true;
        }
    }
    false
}

fn solve(n: usize, adj: &[Vec<usize>]) -> bool {
    let mut visited = vec![false; n];
    for v in 0..n {
        if adj[v].len() >= 3 {
            return true;
        }
        if !visited[v] && dfs(adj, &mut visited, v, None) {
            return true;
        }
    }
    false
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![]; n];
    for (a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }

    println!("{}", if solve(n, &adj) { "No" } else { "Yes" });
}
