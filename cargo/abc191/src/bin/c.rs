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
        fld: [Chars; h],
    }
    let mut count = 0;
    for r in 0..h - 1 {
        for c in 0..w - 1 {
            let mut black = 0;
            for &dr in &[0, 1] {
                for &dc in &[0, 1] {
                    if fld[r + dr][c + dc] == '#' {
                        black += 1;
                    }
                }
            }
            match black {
                1 | 3 => {
                    count += 1;
                }
                _ => (),
            }
        }
    }
    println!("{}", count);
}
