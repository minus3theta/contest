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

fn chmin<T: Ord + Copy>(src: T, dst: &mut T) {
    *dst = (*dst).min(src);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n],
    }
    let inf = 1 << 30;
    let mut dp = vec![vec![vec![inf; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;
    for (i, &(a, b)) in ab.iter().enumerate() {
        for p in 0..=x {
            for q in 0..=y {
                chmin(dp[i][p][q], &mut dp[i + 1][p][q]);
                chmin(
                    dp[i][p.saturating_sub(a)][q.saturating_sub(b)] + 1,
                    &mut dp[i + 1][p][q],
                );
            }
        }
    }
    let ans = dp[n][x][y];

    println!("{}", if ans == inf { -1 } else { ans });
}
