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

// a * b + c = n
// a * b = n - c
// a * b <= n - 1

#[fastout]
fn main() {
    input! {
        n: i32,
    }
    let ans: i32 = (1..n).map(|a| (n - 1) / a).sum();

    println!("{}", ans);
}
