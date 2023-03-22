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
        a: i64,
        x: i64,
        m: i64,
    }
    let c = vec![vec![1, 1], vec![0, a]];
    let c = mat_pow(&c, x, m);
    let v = mat_mult(&c, &[vec![0], vec![1]], m);

    println!("{}", v[0][0]);
}

fn mat_mult(x: &[Vec<i64>], y: &[Vec<i64>], m: i64) -> Vec<Vec<i64>> {
    (0..x.len())
        .map(|i| {
            (0..y[0].len())
                .map(|j| (0..y.len()).map(|k| x[i][k] * y[k][j] % m).sum::<i64>() % m)
                .collect()
        })
        .collect()
}

fn mat_pow(x: &[Vec<i64>], n: i64, m: i64) -> Vec<Vec<i64>> {
    if n == 1 {
        return x.to_vec();
    }
    if n % 2 == 1 {
        let y = mat_pow(x, n - 1, m);
        mat_mult(x, &y, m)
    } else {
        let y = mat_pow(x, n / 2, m);
        mat_mult(&y, &y, m)
    }
}
