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
        y: i32,
    }
    if y == 0 {
        println!("ERROR");
    } else {
        let z = x * 100 / y;
        let int = z / 100;
        let frac = z % 100;
        println!("{}.{:02}", int, frac);
    }
}
