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
        n: Chars,
    }
    let len = n.len();
    let ans = (0..1 << len)
        .filter_map(|pat| {
            let mut left = vec![];
            let mut right = vec![];
            for (i, &digit) in n.iter().enumerate() {
                if (pat >> i) & 1 != 0 {
                    left.push(digit);
                } else {
                    right.push(digit);
                }
            }
            if left.is_empty()
                || left.iter().all(|&d| d == '0')
                || right.is_empty()
                || right.iter().all(|&d| d == '0')
            {
                return None;
            }
            left.sort();
            right.sort();
            let left: i64 = left.into_iter().rev().collect::<String>().parse().unwrap();
            let right: i64 = right.into_iter().rev().collect::<String>().parse().unwrap();
            Some(left * right)
        })
        .max()
        .unwrap();

    println!("{}", ans);
}
