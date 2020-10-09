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
        d: i64,
        ps: [(i64, i64); n],
    }
    let ans = ps
        .into_iter()
        .filter(|&(x, y)| x * x + y * y <= d * d)
        .count();

    println!("{}", ans);
}
