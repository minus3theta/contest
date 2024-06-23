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
        s: String,
    }
    let n = s.len();
    let upper = s.chars().filter(|&c| c.is_ascii_uppercase()).count();
    if upper > n - upper {
        println!("{}", s.to_ascii_uppercase());
    } else {
        println!("{}", s.to_ascii_lowercase());
    }
}
