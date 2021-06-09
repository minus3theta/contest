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

use petgraph::graph::UnGraph;
use petgraph::visit::depth_first_search;
use petgraph::visit::DfsEvent;

// Returns (# nodes, internal sum, sum of distance to root)
fn solve(child: &Vec<Vec<usize>>, v: usize) -> (i64, i64, i64) {
    let sub = child[v].iter().map(|&c| solve(child, c)).collect_vec();
    let nodes: i64 = sub.iter().map(|&(n, _, _)| n).sum();
    let internal = sub
        .iter()
        .map(|&(n, i, r)| i + (r + n) * (nodes + 1 - n))
        .sum();
    let dist_root: i64 = sub.iter().map(|&(n, _, r)| r + n).sum();
    (nodes + 1, internal, dist_root)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n-1],
    }
    let graph = UnGraph::<(), (), usize>::from_edges(&es);
    let mut child = vec![vec![]; n];
    depth_first_search(&graph, Some(0.into()), |event| match event {
        DfsEvent::TreeEdge(u, v) => {
            child[u.index()].push(v.index());
        }
        _ => (),
    });
    let (_, ans, _) = solve(&child, 0);

    println!("{}", ans);
}
