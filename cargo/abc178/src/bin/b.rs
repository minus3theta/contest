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
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let ans = [a, b]
        .iter()
        .cartesian_product(&[c, d])
        .map(|(&x, &y)| x * y)
        .max()
        .unwrap();

    println!("{}", ans);
}
