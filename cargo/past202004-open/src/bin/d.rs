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

fn substrings(s: &str, k: usize) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    for i in 0..=s.len() - k {
        for mask in 0..(1 << k) {
            let mut sub = String::new();
            for j in 0..k {
                if mask & (1 << j) != 0 {
                    sub.push_str(&s[i + j..i + j + 1]);
                } else {
                    sub.push('.');
                }
            }
            set.insert(sub);
        }
    }
    set
}

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = (1..=3)
        .map(|k| {
            if k <= s.len() {
                substrings(&s, k).len()
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("{}", ans);
}
