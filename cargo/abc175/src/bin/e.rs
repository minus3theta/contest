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

fn chmax<T: Ord + Copy>(y: T, x: &mut T) {
    *x = cmp::max(*x, y);
}

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        item: [(usize, usize, i64); k],
    }
    let mut field = vec![vec![0; c + 1]; r + 1];
    for (i, j, v) in item.into_iter() {
        field[i][j] = v;
    }
    let mut dp = vec![vec![vec![0; 4]; c + 1]; r + 1];
    for i in 1..=r {
        for j in 1..=c {
            let up = *dp[i - 1][j].iter().max().unwrap();
            chmax(up, &mut dp[i][j][0]);
            chmax(up + field[i][j], &mut dp[i][j][1]);
            for k in 0..=3 {
                chmax(dp[i][j - 1][k], &mut dp[i][j][k]);
                if let Some(l) = k.checked_sub(1) {
                    chmax(dp[i][j - 1][l] + field[i][j], &mut dp[i][j][k]);
                }
            }
        }
    }

    println!("{}", dp[r][c].iter().max().unwrap());
}
