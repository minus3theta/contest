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
        k: i64,
        a: [i64; n],
    }
    let mut l = 0;
    let mut r = *a.iter().max().unwrap();
    while l + 1 != r {
        let m = (l + r) / 2;
        let cuts: i64 = a.iter().map(|&x| (x + m - 1) / m - 1).sum();
        if cuts <= k {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);
}
