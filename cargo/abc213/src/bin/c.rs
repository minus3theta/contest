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

pub fn compress<T>(array: &[T]) -> std::collections::BTreeMap<T, usize>
where
    T: Clone + PartialEq + Ord,
{
    let mut array = array.to_vec();
    array.sort();
    array.dedup();
    array.into_iter().enumerate().map(|(i, a)| (a, i)).collect()
}

#[fastout]
fn main() {
    input! {
        _h: i64,
        _w: i64,
        n: usize,
        cards: [(i64, i64); n],
    }
    let rows = cards.iter().map(|&(r, _)| r).collect_vec();
    let rows = compress(&rows);
    let cols = cards.iter().map(|&(_, c)| c).collect_vec();
    let cols = compress(&cols);
    for &(a, b) in &cards {
        println!("{} {}", rows[&a] + 1, cols[&b] + 1);
    }
}
