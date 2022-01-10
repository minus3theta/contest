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
        x: Chars,
        n: usize,
        mut s: [Chars; n],
    }
    let rev = x
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i))
        .collect::<BTreeMap<_, _>>();
    s.sort_by_cached_key(|v| v.iter().map(|c| rev[c]).collect_vec());
    for s in s {
        println!("{}", s.into_iter().collect::<String>());
    }
}
