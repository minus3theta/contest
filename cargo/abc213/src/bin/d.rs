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

fn dfs(adj: &[Vec<usize>], v: usize, prev: Option<usize>, ret: &mut Vec<usize>) {
    ret.push(v + 1);
    for &u in &adj[v] {
        if prev == Some(u) {
            continue;
        }
        dfs(adj, u, Some(v), ret);
        ret.push(v + 1);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n-1],
    }
    let mut adj = vec![vec![]; n];
    for (u, v) in es {
        adj[u].push(v);
        adj[v].push(u);
    }
    for a in &mut adj {
        a.sort();
    }
    let mut ret = vec![];
    dfs(&adj, 0, None, &mut ret);

    println!("{}", ret.iter().join(" "));
}
