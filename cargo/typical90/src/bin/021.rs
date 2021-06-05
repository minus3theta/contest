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

use petgraph::algo::kosaraju_scc;
use petgraph::graph::DiGraph;

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }
    let graph = DiGraph::<(), (), usize>::from_edges(&es);
    let scc = kosaraju_scc(&graph);

    println!(
        "{}",
        scc.iter()
            .map(|v| v.len() * (v.len() - 1) / 2)
            .sum::<usize>()
    );
}
