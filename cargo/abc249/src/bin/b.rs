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
        mut s: Chars,
    }
    let up = s.iter().any(|c| c.is_ascii_uppercase());
    let low = s.iter().any(|c| c.is_ascii_lowercase());
    let n = s.len();
    s.sort();
    s.dedup();
    let unique = s.len() == n;

    println!("{}", if up && low && unique { "Yes" } else { "No" });
}
