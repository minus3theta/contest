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
        aa: [i64; n],
    }
    let mut sum = vec![0i64; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + aa[i];
    }
    let total = sum[n];
    let mut ans = std::i64::MAX;
    for left in 0..n {
        let mut l = left;
        let mut r = n + 1;
        while l + 1 != r {
            let m = (l + r) / 2;
            let x = sum[m] - sum[left];
            let y = total - x;
            if x <= y {
                l = m;
            } else {
                r = m;
            }
        }
        let xl = sum[l] - sum[left];
        let yl = total - xl;
        ans = cmp::min(ans, (xl - yl).abs());
        if r <= n {
            let xr = sum[r] - sum[left];
            let yr = total - xr;
            ans = cmp::min(ans, (xr - yr).abs());
        }
    }

    println!("{}", ans);
}
