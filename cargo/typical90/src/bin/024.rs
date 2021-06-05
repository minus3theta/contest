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
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let min_op: i64 = a.into_iter().zip(b).map(|(x, y)| (x - y).abs()).sum();

    println!(
        "{}",
        if min_op <= k && min_op % 2 == k % 2 {
            "Yes"
        } else {
            "No"
        }
    );
}
