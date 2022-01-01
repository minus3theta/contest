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
        h: usize,
        w: usize,
        field: [Chars; h],
    }
    let mut dp = vec![vec![0; w + 1]; h + 1];
    for (r, row) in field.iter().enumerate().rev() {
        for (c, &cell) in row.iter().enumerate().rev() {
            if cell == '.' {
                dp[r][c] = dp[r + 1][c].max(dp[r][c + 1]) + 1;
            }
        }
    }

    println!("{}", dp[0][0]);
}
