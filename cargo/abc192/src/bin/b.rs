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
        s: Chars,
    }
    println!(
        "{}",
        if s.iter().enumerate().all(|(i, &c)| if i % 2 == 0 {
            c.is_ascii_lowercase()
        } else {
            c.is_ascii_uppercase()
        }) {
            "Yes"
        } else {
            "No"
        }
    )
}
