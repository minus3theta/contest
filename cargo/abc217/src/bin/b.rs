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
        s: [String; 3],
    }
    let mut set: BTreeSet<_> = vec!["ABC", "ARC", "AGC", "AHC"].into_iter().collect();
    for s in &s {
        set.remove(s.as_str());
    }

    println!("{}", set.iter().next().unwrap());
}
