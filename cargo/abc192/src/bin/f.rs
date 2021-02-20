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

const INF: i64 = 1 << 62;

fn chmax<T: Ord + Copy>(y: T, x: &mut T) {
    *x = cmp::max(*x, y);
}

fn chmin<T: Ord + Copy>(y: T, x: &mut T) {
    *x = cmp::min(*x, y);
}

fn solve_sub(a: &[i64], x: i64, k: usize) -> i64 {
    let n = a.len();
    let mut dp = vec![vec![vec![-INF; k]; n + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..n {
            for rem in 0..k {
                chmax(dp[i][j][rem], &mut dp[i + 1][j][rem]);
                let next_rem = (rem + a[i] as usize) % k;
                chmax(dp[i][j][rem] + a[i], &mut dp[i + 1][j + 1][next_rem]);
            }
        }
    }
    let target_mod = x as usize % k;
    let init = dp[n][k][target_mod];
    if init >= 0 {
        assert_eq!(init as usize % k, target_mod);
        (x - init) / k as i64
    } else {
        INF
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }
    let mut ans = INF;
    for k in 1..=n {
        chmin(solve_sub(&a, x, k), &mut ans);
    }
    println!("{}", ans);
}
