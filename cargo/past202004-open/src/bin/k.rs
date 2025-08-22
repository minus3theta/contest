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

const INF: i64 = 1 << 60;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [i64; n],
        d: [i64; n],
    }
    let mut cost = vec![vec![INF; n + 1]; n + 1];
    cost[0][0] = 0;
    for i in 0..n {
        for left in 0..=n {
            if s[i] == '(' {
                if left < n {
                    cost[i + 1][left + 1] = cost[i + 1][left + 1].min(cost[i][left]);
                }
                if left > 0 {
                    cost[i + 1][left - 1] = cost[i + 1][left - 1].min(cost[i][left] + c[i]);
                }
            } else {
                if left > 0 {
                    cost[i + 1][left - 1] = cost[i + 1][left - 1].min(cost[i][left]);
                }
                if left < n {
                    cost[i + 1][left + 1] = cost[i + 1][left + 1].min(cost[i][left] + c[i]);
                }
            }
            cost[i + 1][left] = cost[i + 1][left].min(cost[i][left] + d[i]);
        }
    }

    println!("{}", cost[n][0]);
}
