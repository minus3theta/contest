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

const MAX: usize = 200;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let mut is_prime = [true; MAX + 1];
    for i in 2..=MAX {
        if !is_prime[i] {
            continue;
        }
        for j in 2.. {
            if i * j > MAX {
                break;
            }
            is_prime[i * j] = false;
        }
    }
    let ans = (a..=b).any(|x| !(c..=d).any(|y| is_prime[x + y]));

    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
