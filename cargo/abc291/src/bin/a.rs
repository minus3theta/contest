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
    let ans = s
        .chars()
        .enumerate()
        .find_map(|(i, c)| {
            if c.is_ascii_uppercase() {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap();

    println!("{}", ans);
}
