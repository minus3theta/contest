#![allow(clippy::needless_range_loop)]

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
        c: [Chars; n],
    }
    let mut dist = vec![vec![u32::MAX; n]; n];
    let mut que = VecDeque::new();
    for i in 0..n {
        dist[i][i] = 0;
        que.push_back((i, i));
    }
    for i in 0..n {
        for j in 0..n {
            if i == j || c[i][j] == '-' {
                continue;
            }
            dist[i][j] = 1;
            que.push_back((i, j));
        }
    }
    while let Some((i, j)) = que.pop_front() {
        for k in 0..n {
            for l in 0..n {
                if c[k][i] == '-' || c[j][l] == '-' || c[k][i] != c[j][l] || dist[k][l] != u32::MAX
                {
                    continue;
                }
                dist[k][l] = dist[i][j] + 2;
                que.push_back((k, l));
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!(
                "{}{}",
                dist[i][j] as i32,
                if j + 1 == n { '\n' } else { ' ' }
            );
        }
    }
}
