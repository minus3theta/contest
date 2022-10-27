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

fn mean(a: &[i64]) -> f64 {
    let mut l = 0.0;
    let mut r = 2e9;
    for _ in 0..100 {
        let m = (l + r) / 2.0;
        let mut dp = vec![vec![-std::f64::INFINITY; 2]; a.len() + 1];
        dp[0][1] = 0.0;
        for i in 0..a.len() {
            dp[i + 1][0] = dp[i][1];
            dp[i + 1][1] = dp[i][0].max(dp[i][1]) + a[i] as f64 - m;
        }
        if dp[a.len()][0].max(dp[a.len()][1]) >= 0.0 {
            l = m;
        } else {
            r = m;
        }
    }
    l
}

fn median(a: &[i64]) -> i64 {
    let mut l = 0;
    let mut r = 1_000_000_001;
    while l + 1 != r {
        let m = (l + r) / 2;
        let mut dp = vec![vec![-1 << 30; 2]; a.len() + 1];
        dp[0][1] = 0;
        for i in 0..a.len() {
            dp[i + 1][0] = dp[i][1];
            dp[i + 1][1] = dp[i][0].max(dp[i][1]) + if a[i] >= m { 1 } else { -1 };
        }
        if dp[a.len()][0].max(dp[a.len()][1]) > 0 {
            l = m;
        } else {
            r = m;
        }
    }
    l
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{:.15}\n{}", mean(&a), median(&a));
}
