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
        k: Usize1,
    }
    let mut all = s
        .iter()
        .permutations(s.len())
        .map(|v| v.into_iter().collect::<String>())
        .collect_vec();
    all.sort();
    all.dedup();

    println!("{}", all[k]);
}
