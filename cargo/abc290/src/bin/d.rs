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

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
        c: [(i64, i64, i64); t],
    }
    for &(n, d, k) in &c {
        let k = k - 1;
        let g = n / gcd(n, d);
        let q = k / g;
        let r = k % g;
        println!("{}", r * d % n + q);
    }
}
