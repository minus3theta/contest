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

fn gcd(x: i64, y: i64) -> i64 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let g = gcd(a, gcd(b, c));

    println!("{}", a / g - 1 + b / g - 1 + c / g - 1);
}
