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

use petgraph::{algo::dijkstra, graph::DiGraph};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut graph = DiGraph::<(), (), usize>::with_capacity(n, m);
    for _ in 0..n {
        graph.add_node(());
    }
    graph.extend_with_edges(&es);
    let mut ans = 0;
    for i in 0..n {
        let dist = dijkstra(&graph, i.into(), None, |_| 1);
        ans += dist.len();
    }

    println!("{}", ans);
}
