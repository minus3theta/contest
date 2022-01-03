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
        art: [(i64, [Usize1]); n],
    }
    let mut required = vec![false; n];
    required[n - 1] = true;
    for i in (0..n).rev() {
        if !required[i] {
            continue;
        }
        for &a in &art[i].1 {
            required[a] = true;
        }
    }
    let ans = (0..n)
        .filter_map(|i| if required[i] { Some(art[i].0) } else { None })
        .sum::<i64>();

    println!("{}", ans);
}
