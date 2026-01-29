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
        p: i32,
        q: i32,
        x: i32,
        y: i32,
    }

    println!(
        "{}",
        if p <= x && x < p + 100 && q <= y && y < q + 100 {
            "Yes"
        } else {
            "No"
        }
    );
}
