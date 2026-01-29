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
    adj: &[Vec<(usize, i64)>],
    s: i64,
    t: i64,
    cost: i64,
    v: usize,
    l: usize,
) -> BTreeSet<usize> {
    if l == 0 {
        if s <= cost && cost <= t {
            return Some(v).into_iter().collect();
        } else {
            return BTreeSet::new();
        }
    }
    let mut ret = BTreeSet::new();
    for &(u, c) in &adj[v] {
        ret.extend(dfs(adj, s, t, cost + c, u, l - 1));
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: i64,
        t: i64,
        es: [(Usize1, Usize1, i64); m],
    }
    let mut adj = vec![vec![]; n];
    for (u, v, c) in es {
        adj[u].push((v, c));
    }
    for ans in dfs(&adj, s, t, 0, 0, l) {
        print!("{} ", ans + 1);
    }
    println!();
}
