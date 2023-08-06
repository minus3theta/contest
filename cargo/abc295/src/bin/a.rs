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
        w: [String; n],
    }
    fn ok(s: String) -> bool {
        matches!(s.as_str(), "and" | "not" | "that" | "the" | "you")
    }

    println!("{}", if w.into_iter().any(ok) { "Yes" } else { "No" });
}
