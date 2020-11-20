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
        v: [i32; 3],
    }
    let mut v: Vec<_> = v.into_iter().zip(&['A', 'B', 'C']).collect();
    v.sort();

    println!("{}", v[1].1);
}
