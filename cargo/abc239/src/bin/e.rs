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

fn dfs(
    v: usize,
    prev: usize,
    adj: &[Vec<usize>],
    x: &[i64],
    max: &mut [Vec<cmp::Reverse<i64>>],
) -> Vec<cmp::Reverse<i64>> {
    let mut bag = vec![cmp::Reverse(x[v])];
    for &u in &adj[v] {
        if u == prev {
            continue;
        }
        let m = dfs(u, v, adj, x, max);
        bag.extend_from_slice(&m);
        bag.sort();
        bag.truncate(20);
    }
    max[v] = bag.clone();
    bag
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [i64; n],
        es: [(Usize1, Usize1); n-1],
        qs: [(Usize1, Usize1); q],
    }
    let mut adj = vec![vec![]; n];
    for &(u, v) in &es {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut max = vec![vec![]; n];
    dfs(0, n, &adj, &x, &mut max);
    for &(v, k) in &qs {
        println!("{}", max[v][k].0);
    }
}
