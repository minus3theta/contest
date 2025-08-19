#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        s: String,
    }
    if let Ok(n) = s.parse::<i32>() {
        println!("{}", n * 2);
    } else {
        println!("error");
    }
}
