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

fn count_paths(p: i64, adj: &[Vec<i64>], x: i64) -> usize {
    let n = adj.len();
    let mut dist = adj.to_vec();
    for row in &mut dist {
        for v in row {
            if *v == -1 {
                *v = x;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| dist[i][j] <= p)
        .count()
}

fn k_or_more_lb(p: i64, adj: &[Vec<i64>], k: usize) -> i64 {
    let mut l = 0;
    let mut r = 1_000_000_002;
    while l + 1 != r {
        let x = (l + r) / 2;
        if count_paths(p, adj, x) >= k {
            l = x;
        } else {
            r = x;
        }
    }
    l
}

#[fastout]
fn main() {
    input! {
        n: usize,
        p: i64,
        k: usize,
        adj: [[i64; n]; n],
    }
    let ub = k_or_more_lb(p, &adj, k);
    let lb = k_or_more_lb(p, &adj, k + 1);
    let inf = 1_000_000_000;
    if lb <= inf && ub > inf {
        println!("Infinity");
    } else {
        println!("{}", ub - lb);
    }
}
