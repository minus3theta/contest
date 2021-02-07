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
        a: [i32; n],
        b: [i32; n],
    }
    let p: i32 = a.into_iter().zip(b.into_iter()).map(|(x, y)| x * y).sum();
    println!("{}", if p == 0 { "Yes" } else { "No" })
}
