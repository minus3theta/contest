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

fn choices(items: &[i64], count: usize) -> Vec<i64> {
    let mut ret: Vec<i64> = items
        .iter()
        .combinations(count)
        .map(|v| v.into_iter().sum())
        .collect();
    ret.sort();
    ret
}

fn count_le(prices: &[i64], p: i64) -> usize {
    // prices[l] <= p
    // prices[r] > p
    let mut l = -1isize;
    let mut r = prices.len() as isize;
    while l + 1 != r {
        let m = (l + r) / 2;
        if prices[m as usize] <= p {
            l = m;
        } else {
            r = m;
        }
    }
    (l + 1) as usize
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: i64,
        a: [i64; n],
    }
    let (first, second) = a.split_at(n / 2);
    let mut ans = 0;
    for first_count in 0..=k.min(first.len()) {
        let second_count = k - first_count;
        let second_choices = choices(&second, second_count);
        for f in choices(&first, first_count) {
            ans += count_le(&second_choices, p - f);
        }
    }

    println!("{}", ans);
}
