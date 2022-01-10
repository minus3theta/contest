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
        x: i32,
    }
    if x < 40 {
        println!("{}", 40 - x);
    } else if x < 70 {
        println!("{}", 70 - x);
    } else if x < 90 {
        println!("{}", 90 - x);
    } else {
        println!("expert");
    }
}
