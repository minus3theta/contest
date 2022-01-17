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
        a: i32,
        b: i32,
    }

    println!(
        "{}",
        match (a, b) {
            (_, 0) => "Gold",
            (0, _) => "Silver",
            _ => "Alloy",
        }
    );
}
