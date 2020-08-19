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

fn solve(cycle: Vec<i64>, k: usize) -> i64 {
    let m = cycle.len();
    let total: i64 = cycle.iter().sum();
    let mut dp = vec![vec![0; m]; m];
    for i in 0..m {
        for j in 1..m {
            dp[i][j] = dp[i][j - 1] + cycle[(i + j) % m];
        }
    }
    let mut ret = -(1i64 << 60);
    if total < 0 || k < m {
        for i in 0..m {
            for j in 1..cmp::min(m, k + 1) {
                ret = cmp::max(ret, dp[i][j]);
            }
        }
    } else {
        let base = total * (k / m - 1) as i64;
        for i in 0..m {
            for j in 1..m {
                ret = cmp::max(ret, base + dp[i][j]);
            }
        }
        let base = total * (k / m) as i64;
        for i in 0..m {
            for j in 0..=k % m {
                ret = cmp::max(ret, base + dp[i][j]);
            }
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
        c: [i64; n],
    }
    let mut ans = -(1i64 << 60);
    let mut used = vec![false; n];
    for i in 0..n {
        if used[i] {
            continue;
        }
        let mut j = i;
        let mut cycle = vec![];
        loop {
            cycle.push(c[j]);
            used[j] = true;
            j = p[j];
            if j == i {
                break;
            }
        }
        ans = cmp::max(ans, solve(cycle, k))
    }

    println!("{}", ans);
}
