#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;

use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        qs: [(i32, usize, usize); q],
    }
    let mut shift = 0;
    let shifted = |i: usize, shift: usize| (i + shift) % n;
    for &(t, x, y) in &qs {
        match t {
            1 => {
                let x = shifted(x - 1, shift);
                let y = shifted(y - 1, shift);
                a.swap(x, y);
            }
            2 => {
                shift = (shift + n - 1) % n;
            }
            _ => {
                let x = shifted(x - 1, shift);
                println!("{}", a[x]);
            }
        }
    }
}
