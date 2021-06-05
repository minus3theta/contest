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
        n: usize,
        ss: [String; n],
    }
    let mut registered = HashSet::new();
    for (i, s) in ss.into_iter().enumerate() {
        if registered.insert(s) {
            println!("{}", i + 1);
        }
    }
}
