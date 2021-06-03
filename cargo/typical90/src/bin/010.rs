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
        cp: [(Usize1, i64); n],
        q: usize,
        lr: [(Usize1, usize); q],
    }

    let mut sum = vec![vec![0; n + 1]; 2];
    for (i, &(c, p)) in cp.iter().enumerate() {
        sum[c][i + 1] = p;
    }
    for v in &mut sum {
        for i in 0..n {
            v[i + 1] += v[i];
        }
    }

    for &(l, r) in &lr {
        println!("{} {}", sum[0][r] - sum[0][l], sum[1][r] - sum[1][l]);
    }
}
