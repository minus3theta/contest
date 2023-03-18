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
        mut x: [i64; 5*n],
    }
    x.sort();
    let x = x.into_iter().skip(n).take(3 * n).sum::<i64>() as f64 / (3 * n) as f64;

    println!("{:.15}", x);
}
