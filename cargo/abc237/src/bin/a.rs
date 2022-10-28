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
        n: i64,
    }

    println!(
        "{}",
        if (std::i32::MIN as i64..=std::i32::MAX as i64).contains(&n) {
            "Yes"
        } else {
            "No"
        }
    );
}
