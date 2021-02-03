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

fn is_clique(s: usize, adj: &Vec<Vec<bool>>) -> bool {
    let n = adj.len();
    (0..n)
        .filter(|&i| s & (1 << i) != 0)
        .tuple_combinations()
        .all(|(a, b)| adj[a][b])
}

fn component(s: usize, adj: &Vec<Vec<bool>>, cache: &[usize]) -> usize {
    if is_clique(s, adj) {
        return 1;
    }
    let mut com = adj.len();
    let mut t = s;
    while t != 0 {
        com = cmp::min(com, cache[t] + cache[s ^ t]);
        t = (t - 1) & s;
    }
    com
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![false; n]; n];
    for &(a, b) in &es {
        adj[a][b] = true;
        adj[b][a] = true;
    }
    let mut count = vec![n; 1 << n];
    for s in 0..1 << n {
        count[s] = component(s, &adj, &count);
    }

    println!("{}", count[(1 << n) - 1]);
}
