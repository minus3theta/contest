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

fn solve(a: &[i64]) -> bool {
    let sum = a.iter().sum::<i64>();
    if sum % 10 != 0 {
        return false;
    }
    let target = sum / 10;
    let n = a.len();
    let mut tail = 0;
    let mut current = 0;
    for head in 0..n {
        while current < target {
            current += a[tail];
            tail = (tail + 1) % n;
        }
        if current == target {
            return true;
        }
        current -= a[head];
    }
    false
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    println!("{}", if solve(&a) { "Yes" } else { "No" });
}
