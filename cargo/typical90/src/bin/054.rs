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
        paper: [[Usize1]; m],
    }
    let mut es = vec![];
    for (i, rs) in paper.iter().enumerate() {
        let paper_id = i + n;
        for &r in rs {
            es.push((r, paper_id, 1));
            es.push((paper_id, r, 0));
        }
    }
    let graph = DiGraph::<(), i64, usize>::from_edges(&es);
    let dist = dijkstra(&graph, 0.into(), None, |e| *e.weight());
    for i in 0..n {
        println!("{}", dist.get(&i.into()).unwrap_or(&-1));
    }
}
