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
        a: [[i64; w]; h],
    }
    let ans = (|| {
        for (i1, i2) in (0..h).tuple_combinations() {
            for (j1, j2) in (0..w).tuple_combinations() {
                if a[i1][j1] + a[i2][j2] > a[i2][j1] + a[i1][j2] {
                    return false;
                }
            }
        }
        true
    })();

    println!("{}", if ans { "Yes" } else { "No" });
}
