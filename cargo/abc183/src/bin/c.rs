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
        k: i64,
        t: [[i64; n]; n],
    }

    println!("{}", dfs(0, 1, 0, &t, k));
}

fn dfs(v: usize, visited: usize, dist: i64, adj: &Vec<Vec<i64>>, k: i64) -> i64 {
    // eprintln!("{}, {:b}, {}", v, visited, dist);
    if visited == (1 << adj.len()) - 1 {
        return if dist + adj[0][v] == k { 1 } else { 0 };
    }
    let mut ret = 0;
    for (u, &d) in adj[v].iter().enumerate() {
        if 1 << u & visited != 0 {
            continue;
        }
        ret += dfs(u, visited | 1 << u, dist + d, adj, k);
    }
    ret
}
