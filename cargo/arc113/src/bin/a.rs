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
        k: i64,
    }
    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k / a {
            ans += k / (a * b);
        }
    }
    println!("{}", ans);
}
