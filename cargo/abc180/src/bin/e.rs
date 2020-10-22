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
        vs: [(i32, i32, i32); n],
    }
    let distance = |i: usize, j: usize| {
        (vs[j].0 - vs[i].0).abs() + (vs[j].1 - vs[i].1).abs() + cmp::max(0, vs[j].2 - vs[i].2)
    };
    let mut dist = vec![vec![1 << 30; n]; 1 << n];
    for i in 1..n {
        dist[1 << i][i] = distance(0, i);
    }
    for visited in 0..1 << n {
        for i in 0..n {
            if visited & 1 << i == 0 {
                continue;
            }
            for j in 0..n {
                if visited & 1 << j != 0 {
                    continue;
                }
                let new_pat = visited | 1 << j;
                dist[new_pat][j] = cmp::min(dist[new_pat][j], dist[visited][i] + distance(i, j));
            }
        }
    }

    println!("{}", dist[(1 << n) - 1][0]);
}
