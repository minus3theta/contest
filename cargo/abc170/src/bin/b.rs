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

fn solve(x: i32, y: i32) -> bool {
    for i in 0..=x {
        if 2 * i + 4 * (x - i) == y {
            return true;
        }
    }
    false
}

#[fastout]
fn main() {
    input! {
      x: i32,
      y: i32,
    }

    println!("{}", if solve(x, y) { "Yes" } else { "No" });
}
