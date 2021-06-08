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
        h: i32,
        w: i32,
    }

    println!(
        "{}",
        if h == 1 || w == 1 {
            h * w
        } else {
            ((h + 1) / 2) * ((w + 1) / 2)
        }
    );
}
