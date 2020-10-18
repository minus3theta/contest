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
        ds: [(i32, i32); n],
    }
    let mut cont = 0;
    let mut max_cont = 0;
    for &(x, y) in &ds {
        if x == y {
            cont += 1;
            max_cont = cmp::max(max_cont, cont);
        } else {
            cont = 0;
        }
    }

    println!("{}", if max_cont >= 3 { "Yes" } else { "No" });
}
