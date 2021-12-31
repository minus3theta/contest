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
        x: i64,
        ball: [[i64]; n],
    }
    let ans = ball
        .iter()
        .map(|v| v.iter())
        .multi_cartesian_product()
        .filter(|v| v.iter().try_fold(1i64, |acc, &&a| acc.checked_mul(a)) == Some(x))
        .count();

    println!("{}", ans);
}
