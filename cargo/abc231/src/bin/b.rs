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
        s: [String; n],
    }
    let mut popl = HashMap::new();
    for s in s {
        *popl.entry(s).or_insert(0) += 1;
    }

    println!("{}", popl.iter().max_by_key(|&(_, &p)| p).unwrap().0);
}
