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

fn farthest(start: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> (usize, usize) {
    let mut dist = 0;
    let mut leaf = start;
    visited[start] = true;
    for &v in &adj[start] {
        if visited[v] {
            continue;
        }
        let (d, l) = farthest(v, adj, visited);
        if d + 1 > dist {
            dist = d + 1;
            leaf = l;
        }
    }
    (dist, leaf)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for &(a, b) in &es {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut visited = vec![false; n];
    let (_, leaf) = farthest(0, &adj, &mut visited);
    let mut visited = vec![false; n];
    let (diameter, _) = farthest(leaf, &adj, &mut visited);
    println!("{}", diameter + 1);
}
