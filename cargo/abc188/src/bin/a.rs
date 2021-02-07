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
        x: i32,
        y: i32,
    }
    let a = cmp::min(x, y);
    let b = cmp::max(x, y);
    println!("{}", if a + 3 > b { "Yes" } else { "No" })
}
