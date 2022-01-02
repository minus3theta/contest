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
        mut x: Usize1,
        a: [Usize1; n],
    }
    let mut known = vec![false; n];
    while !known[x] {
        known[x] = true;
        x = a[x];
    }

    println!("{}", known.iter().filter(|&&k| k).count());
}
