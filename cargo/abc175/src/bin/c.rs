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
        x: i64,
        k: i64,
        d: i64,
    }
    let x = x.abs();
    let min = x - k.checked_mul(d).unwrap_or(1 << 60);
    let mut ans = min.abs();
    let rem = (x + (k % 2) * d) % (2 * d);
    if rem >= min {
        ans = cmp::min(ans, rem);
    }
    let neg = rem - 2 * d;
    if neg >= min {
        ans = cmp::min(ans, neg.abs());
    }

    println!("{}", ans);
}
