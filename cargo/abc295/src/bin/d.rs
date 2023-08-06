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
    let mut set = 0;
    let mut popl = BTreeMap::new();
    popl.insert(0, 1);
    for &c in &s {
        set ^= 1 << (c as u8 - b'0');
        *popl.entry(set).or_insert(0i64) += 1;
    }

    println!("{}", popl.values().map(|p| p * (p - 1) / 2).sum::<i64>());
}
