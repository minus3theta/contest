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

fn includes(set: &BTreeSet<char>, word: &str) -> bool {
    word.chars().all(|c| set.contains(&c))
}

#[fastout]
fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: String,
        t: String,
        q: usize,
        ws: [String; q],
    }
    let s: BTreeSet<_> = s.chars().collect();
    let t: BTreeSet<_> = t.chars().collect();
    for w in ws {
        let ans = match (includes(&s, &w), includes(&t, &w)) {
            (true, false) => "Takahashi",
            (false, true) => "Aoki",
            _ => "Unknown",
        };
        println!("{}", ans);
    }
}
