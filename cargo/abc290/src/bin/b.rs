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
        _n: usize,
        mut k: usize,
        s: Chars,
    }
    for &c in &s {
        if c == 'o' && k > 0 {
            print!("o");
            k -= 1;
        } else {
            print!("x");
        }
    }

    println!();
}
