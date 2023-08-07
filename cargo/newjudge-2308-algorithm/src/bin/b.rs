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
        mut x: i64,
        k: u32,
    }
    for _ in 0..k {
        let d = x % 10;
        x /= 10;
        if d >= 5 {
            x += 1;
        }
    }

    println!("{}", x * 10i64.pow(k));
}
