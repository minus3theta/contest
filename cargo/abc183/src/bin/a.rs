#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        x: i32,
    }

    println!("{}", cmp::max(x, 0));
}
