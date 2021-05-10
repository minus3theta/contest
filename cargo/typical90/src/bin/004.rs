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
        a: [[i32; w]; h],
    }
    let row_sum: Vec<i32> = a.iter().map(|r| r.iter().sum()).collect();
    let col_sum: Vec<i32> = (0..w).map(|c| a.iter().map(|r| r[c]).sum()).collect();

    for i in 0..h {
        for j in 0..w {
            if j != 0 {
                print!(" ");
            }
            print!("{}", row_sum[i] + col_sum[j] - a[i][j]);
        }
        println!();
    }
}
