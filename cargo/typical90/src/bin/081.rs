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

const M: usize = 5000;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        students: [(Usize1, Usize1); n],
    }
    let mut sum = vec![vec![0; M + 1]; M + 1];
    for &(a, b) in &students {
        sum[a][b] += 1;
        sum[M.min(a + k + 1)][b] -= 1;
        sum[a][M.min(b + k + 1)] -= 1;
        sum[M.min(a + k + 1)][M.min(b + k + 1)] += 1;
    }
    for i in 0..=M {
        for j in 0..M {
            sum[i][j + 1] += sum[i][j];
        }
    }
    for i in 0..M {
        for j in 0..=M {
            sum[i + 1][j] += sum[i][j];
        }
    }

    println!("{}", sum.iter().flat_map(|r| r.iter()).max().unwrap());
}
