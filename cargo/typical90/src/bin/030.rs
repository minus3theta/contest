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
        k: i32,
    }
    let mut factors = vec![0; n + 1];
    for i in 2..=n {
        if factors[i] != 0 {
            continue;
        }
        for j in 1.. {
            if i * j > n {
                break;
            }
            factors[i * j] += 1;
        }
    }

    println!("{}", factors.iter().filter(|&&f| f >= k).count());
}
