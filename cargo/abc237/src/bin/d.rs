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
        s: Chars,
    }

    let mut left = vec![];
    let mut right = vec![];
    for (i, &c) in (0..).zip(&s) {
        if c == 'R' {
            left.push(i);
        } else {
            right.push(i);
        }
    }
    left.push(n);

    for &x in left.iter().chain(right.iter().rev()) {
        print!("{} ", x);
    }

    println!();
}
