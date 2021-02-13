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

fn size(i: (i64, i64)) -> i64 {
    i.1 - i.0
}

fn intersection(i: (i64, i64), j: (i64, i64)) -> (i64, i64) {
    let l = cmp::max(i.0, j.0);
    let r = cmp::min(i.1, j.1);
    if l > r {
        (0, 0)
    } else {
        (l, r)
    }
}

#[fastout]
fn main() {
    input! {
        b: i64,
        c: i64,
    }
    let i = (-b - (c - 1) / 2, -b + (c - 1) / 2 + 1);
    let j = (b - c / 2, b + (c - 2) / 2 + 1);
    let k = intersection(i, j);
    println!("{}", size(i) + size(j) - size(k));
}
