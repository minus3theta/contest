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
        y: i64,
        a: i64,
        b: i64,
    }
    let mut exp = 0;
    while x < y / a && x < (x + b) / a {
        x *= a;
        exp += 1;
    }
    let rest = y - x - 1;
    exp += rest / b;

    println!("{}", exp);
}
