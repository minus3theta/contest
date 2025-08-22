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
        mut s: [Chars; n],
    }
    for i in (0..n - 1).rev() {
        let below = s[i + 1].clone();
        for (j, c) in s[i].iter_mut().enumerate() {
            if *c != '#' {
                continue;
            }
            if below[j - 1] == 'X' || below[j] == 'X' || below[j + 1] == 'X' {
                *c = 'X';
            }
        }
    }

    for r in &s {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
}
