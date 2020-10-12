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
        k: usize,
        cs: [Chars; h],
    }
    let mut ans = 0;
    for row_pat in 0..(1 << h) {
        for col_pat in 0..(1 << w) {
            let mut black = 0;
            for row in 0..h {
                for col in 0..w {
                    if cs[row][col] == '#' && row_pat >> row & 1 != 0 && col_pat >> col & 1 != 0 {
                        black += 1;
                    }
                }
            }
            if black == k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
