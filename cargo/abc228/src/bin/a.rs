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
        s: i32,
        t: i32,
        x: i32,
    }
    let ans = s <= x && x < t || t < s && (x < t || s <= x);

    println!("{}", if ans { "Yes" } else { "No" });
}
