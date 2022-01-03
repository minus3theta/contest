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
    }
    let mut l = 0;
    let mut r = 1i64 << 60;
    while l + 1 != r {
        let m = (l + r) / 2;
        let sum: i64 = a.iter().map(|&a| a.min(m)).sum();
        if k.saturating_mul(m) <= sum {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}
