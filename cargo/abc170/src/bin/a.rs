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

#[fastout]
fn main() {
    input! {
      x: [i32; 5],
    }
    for i in 0..5 {
        if x[i] == 0 {
            println!("{}", i + 1);
        }
    }
}
