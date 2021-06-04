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
        a: [i64; 2 * n],
    }
    let mut cost = vec![vec![1i64 << 60; 2 * n + 1]; 2 * n + 1];
    for i in 0..=2 * n {
        cost[i][i] = 0;
    }
    for pairs in 1..=n {
        let len = pairs * 2;
        for i in 0..=2 * n - len {
            // calculate cost[i][i+len]
            cost[i][i + len] = (a[i] - a[i + len - 1]).abs() + cost[i + 1][i + len - 1];
            for first_pairs in 1..pairs {
                let mid = i + first_pairs * 2;
                cost[i][i + len] = cost[i][i + len].min(cost[i][mid] + cost[mid][i + len]);
            }
        }
    }

    println!("{}", cost[0][2 * n]);
}
