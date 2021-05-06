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

fn correct(n: usize, s: i32) -> bool {
    let mut close = 0;
    for i in 0..n {
        if s >> i & 1 == 1 {
            close += 1;
        } else {
            close -= 1;
            if close < 0 {
                return false;
            }
        }
    }
    close == 0
}

fn put(n: usize, s: i32) {
    for i in (0..n).rev() {
        if s >> i & 1 == 1 {
            print!(")");
        } else {
            print!("(");
        }
    }
    println!();
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    for s in 0..(1 << n) {
        if correct(n, s) {
            put(n, s);
        }
    }
}
