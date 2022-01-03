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

fn solve(m: usize, b: &[Vec<usize>]) -> bool {
    let start = b[0][0];
    let col = (start - 1) % 7;
    if col + m > 7 {
        return false;
    }
    for (i, row) in b.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            if x != start + i * 7 + j {
                return false;
            }
        }
    }
    true
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    }

    println!("{}", if solve(m, &b) { "Yes" } else { "No" });
}
