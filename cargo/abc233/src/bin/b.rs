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
        l: Usize1,
        r: usize,
        s: String,
    }

    println!(
        "{}{}{}",
        &s[..l],
        &s[l..r].chars().rev().collect::<String>(),
        &s[r..]
    );
}
