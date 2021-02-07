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
        c: i64,
        ss: [(i64, i64, i64); n],
    }
    let mut diffs = BTreeMap::new();
    for &(a, b, c) in &ss {
        *diffs.entry(a).or_insert(0) += c;
        *diffs.entry(b + 1).or_insert(0) -= c;
    }
    let mut current = 0;
    let mut prev_day = 0;
    let mut sum = 0;
    for (&day, &diff) in &diffs {
        sum += cmp::min(current, c) * (day - prev_day);
        current += diff;
        prev_day = day;
    }
    println!("{}", sum);
}
