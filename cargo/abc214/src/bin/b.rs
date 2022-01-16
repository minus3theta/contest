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
        s: i32,
        t: i32,
    }
    let ans = (0..3)
        .map(|_| 0..=s)
        .multi_cartesian_product()
        .filter(|v| v.iter().sum::<i32>() <= s && v.iter().product::<i32>() <= t)
        .count();

    println!("{}", ans);
}
