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

#[fastout]
fn main() {
    input! {
        n: usize,
        ps: [(i64, i64); n],
    }
    let sq = ps
        .iter()
        .tuple_combinations()
        .map(|(&(x0, y0), &(x1, y1))| (x1 - x0).pow(2) + (y1 - y0).pow(2))
        .max()
        .unwrap();

    println!("{:.15}", (sq as f64).sqrt());
}
