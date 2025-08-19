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
        m: usize,
        sc: [(String, i64); m],
    }
    let sc = sc
        .into_iter()
        .map(|(s, c)| {
            let set: usize = s
                .chars()
                .enumerate()
                .map(|(i, x)| if x == 'Y' { 1_usize << i } else { 0 })
                .sum();
            (set, c)
        })
        .collect::<Vec<_>>();
    let mut cost = vec![vec![i64::MAX; 1 << n]; m + 1];
    cost[0][0] = 0;
    for (i, &(s, c)) in sc.iter().enumerate() {
        for j in 0..(1 << n) {
            cost[i + 1][j] = cost[i + 1][j].min(cost[i][j]);
            cost[i + 1][j | s] = cost[i + 1][j | s].min(cost[i][j].saturating_add(c));
        }
    }
    let ans = cost[m][(1 << n) - 1];
    println!("{}", if ans == i64::MAX { -1 } else { ans });
}
