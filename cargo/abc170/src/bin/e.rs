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

use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

const K: usize = 200_000;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut ab: [(i64, Usize1); n],
        query: [(Usize1, Usize1); q],
    }
    let mut kg = vec![BTreeSet::new(); K];
    for (i, &(a, b)) in ab.iter().enumerate() {
        kg[b].insert((a, i));
    }
    let mut heap = BinaryHeap::new();
    for k in 0..K {
        if let Some(&(a, _)) = kg[k].iter().next_back() {
            heap.push(Reverse((a, k)));
        }
    }
    for &(c, d) in &query {
        let (a, b) = ab[c];
        ab[c].1 = d;
        kg[b].remove(&(a, c));
        kg[d].insert((a, c));
        if let Some(&(a, _)) = kg[b].iter().next_back() {
            heap.push(Reverse((a, b)));
        }
        if let Some(&(a, _)) = kg[d].iter().next_back() {
            heap.push(Reverse((a, d)));
        }
        loop {
            let &Reverse((r, k)) = heap.peek().unwrap();
            if kg[k].iter().next_back().map(|p| p.0 != r).unwrap_or(true) {
                heap.pop();
                continue;
            }
            println!("{}", r);
            break;
        }
    }
}
