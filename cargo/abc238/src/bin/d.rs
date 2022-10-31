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

fn solve(a: u64, s: u64) -> bool {
    let x = a;
    if let Some(d) = s.checked_sub(x * 2) {
        d & a == 0
    } else {
        false
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
        case: [(u64, u64); t],
    }
    for &(a, s) in &case {
        println!("{}", if solve(a, s) { "Yes" } else { "No" })
    }
}
