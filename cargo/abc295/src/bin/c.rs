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
        a: [i64; n],
    }
    let mut popl = BTreeMap::new();
    for &x in &a {
        *popl.entry(x).or_insert(0) += 1;
    }

    println!("{}", popl.values().map(|&p| p / 2).sum::<i64>());
}
