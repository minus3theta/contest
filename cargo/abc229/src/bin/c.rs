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
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut w: i64,
        mut cheese: [(i64, i64); n],
    }
    cheese.sort_by_key(|&(a, _)| Reverse(a));
    let mut value = 0;
    for &(a, b) in &cheese {
        let weight = b.min(w);
        value += a * weight;
        w -= weight;
    }

    println!("{}", value);
}
