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

fn cost(a: &[i64], b: i64) -> i64 {
    // a[l] <= b
    let mut l = -1isize;
    // b < a[r]
    let mut r = a.len() as isize;
    while l + 1 < r {
        let m = (l + r) / 2;
        if a[m as usize] <= b {
            l = m;
        } else {
            r = m;
        }
    }
    let mut ret = 1 << 60;
    if l >= 0 {
        ret = ret.min((a[l as usize] - b).abs());
    }
    if r < a.len() as isize {
        ret = ret.min((a[r as usize] - b).abs());
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        b: [i64; q],
    }
    a.sort();
    for &x in &b {
        println!("{}", cost(&a, x));
    }
}
