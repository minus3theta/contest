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
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut op = 0;
    for r in 0..h - 1 {
        for c in 0..w - 1 {
            let d = b[r][c] - a[r][c];
            op += d.abs();
            for dr in 0..2 {
                for dc in 0..2 {
                    a[r + dr][c + dc] += d;
                }
            }
        }
    }
    if a == b {
        println!("Yes\n{}", op);
    } else {
        println!("No");
    }
}
