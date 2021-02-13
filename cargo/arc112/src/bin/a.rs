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

fn solve(s: i64) -> i64 {
    if s <= 0 {
        0
    } else {
        s * (s + 1) / 2
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
        cases: [(i64, i64); t],
    }
    for &(l, r) in &cases {
        println!("{}", solve(r - l * 2 + 1));
    }
}
