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

fn solve(k: i64, aa: &[i64]) -> i64 {
    let a_len: i64 = aa.iter().sum();
    if a_len <= k {
        return aa.iter().map(|&a| a * (a + 1) / 2).sum();
    }
    let mut l = 1;
    let mut r = aa.iter().max().unwrap() + 1;
    while l + 1 != r {
        let m = (l + r) / 2;
        if count_leq(aa, m) <= k {
            r = m;
        } else {
            l = m;
        }
    }
    let ret: i64 = aa.iter().map(|&a| (a + r) * (a - r + 1).max(0) / 2).sum();
    let rest = k - count_leq(aa, r);
    ret + rest * (r - 1)
}

fn count_leq(aa: &[i64], m: i64) -> i64 {
    aa.iter().map(|&a| (a - m + 1).max(0)).sum()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        aa: [i64; n],
    }

    println!("{}", solve(k, &aa));
}
