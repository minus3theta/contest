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
        s: Chars,
    }
    let mut ss = vec![];
    let mut iter = s.iter();
    while let Some(&first) = iter.next() {
        let mut current = String::new();
        current.push(first.to_ascii_lowercase());
        for &next in iter.by_ref() {
            current.push(next.to_ascii_lowercase());
            if next.is_ascii_uppercase() {
                break;
            }
        }
        ss.push(current);
    }
    ss.sort();
    for s in ss {
        let first = &s[0..1];
        let mid = &s[1..s.len() - 1];
        let last = &s[s.len() - 1..];
        print!(
            "{}{}{}",
            first.to_ascii_uppercase(),
            mid,
            last.to_ascii_uppercase()
        );
    }
    println!();
}
