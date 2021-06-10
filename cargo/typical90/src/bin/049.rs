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

use petgraph::algo::min_spanning_tree;
use petgraph::data::Element;
use petgraph::graph::UnGraph;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        item: [(i64, Usize1, usize); m],
    }
    let mut graph = UnGraph::<(), _, usize>::with_capacity(n + 1, m);
    for _ in 0..=n {
        graph.add_node(());
    }
    graph.extend_with_edges(item.into_iter().map(|(c, l, r)| (l, r, c)));
    let mst = min_spanning_tree(&graph);
    let edges = mst
        .filter_map(|elem| match elem {
            Element::Edge { weight, .. } => Some(weight),
            _ => None,
        })
        .collect_vec();

    println!(
        "{}",
        if edges.len() == n {
            edges.iter().sum()
        } else {
            -1i64
        }
    );
}
