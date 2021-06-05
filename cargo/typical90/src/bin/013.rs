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

use petgraph::algo::dijkstra;
use petgraph::graph::UnGraph;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1, i64); m],
    }
    let g = UnGraph::<(), i64, usize>::from_edges(&es);
    let dist0 = dijkstra(&g, 0.into(), None, |e| *e.weight());
    let dist1 = dijkstra(&g, (n - 1).into(), None, |e| *e.weight());

    for i in 0..n {
        println!("{}", dist0[&i.into()] + dist1[&i.into()]);
    }
}
