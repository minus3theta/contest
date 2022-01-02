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
        s1: String,
        s2: String,
    }
    let ans = match (s1.as_str(), s2.as_str()) {
        ("#.", ".#") => false,
        (".#", "#.") => false,
        _ => true,
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
