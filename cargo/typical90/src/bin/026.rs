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
use petgraph::visit::{depth_first_search, DfsEvent};

#[fastout]
fn main() {
    input! {
        n: usize,
        es: [(u32, u32); n - 1],
    }
    let g = UnGraph::<(), ()>::from_edges(&es);
    let mut color = vec![false; n];
    depth_first_search(&g, Some(1.into()), |event| match event {
        DfsEvent::TreeEdge(u, v) => {
            color[v.index() - 1] = !color[u.index() - 1];
        }
        _ => (),
    });
    let selected = color.iter().filter(|&&c| c).count() >= n / 2;
    for (v, _) in color
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b == selected)
        .take(n / 2)
    {
        print!("{} ", v + 1);
    }
    println!();
}
