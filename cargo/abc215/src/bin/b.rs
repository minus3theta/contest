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

fn solve(n: u64) -> u32 {
    for k in 0.. {
        if 1 << (k + 1) > n {
            return k;
        }
    }
    unreachable!()
}

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    println!("{}", solve(n));
}
