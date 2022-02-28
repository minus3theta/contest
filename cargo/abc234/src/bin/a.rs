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

fn f(x: i64) -> i64 {
    x * x + 2 * x + 3
}

#[fastout]
fn main() {
    input! {
        t: i64,
    }

    println!("{}", f(f(f(t) + t) + f(f(t))));
}
