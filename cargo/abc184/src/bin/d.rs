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
        a0: usize,
        b0: usize,
        c0: usize,
    }
    let t0 = a0 + b0 + c0;
    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    dp[a0][b0][c0] = 1.0;
    for total in 1usize..=100 + 99 + 99 {
        for a in 0..=cmp::min(99, total) {
            for b in 0..=cmp::min(99, total.saturating_sub(a)) {
                if let Some(c) = total.checked_sub(a + b) {
                    if c >= 100 {
                        continue;
                    }
                    let p = dp[a][b][c];
                    dp[a + 1][b][c] += p * a as f64 / total as f64;
                    dp[a][b + 1][c] += p * b as f64 / total as f64;
                    dp[a][b][c + 1] += p * c as f64 / total as f64;
                }
            }
        }
    }
    let mut ans = 0.0;
    for x in 0..100 {
        for y in 0..100 {
            if let Some(step) = (100usize + x + y).checked_sub(t0) {
                ans += (dp[100][x][y] + dp[x][100][y] + dp[x][y][100]) * step as f64;
            }
        }
    }

    println!("{:.15}", ans);
}
