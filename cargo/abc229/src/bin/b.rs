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
        a: Chars,
        b: Chars,
    }
    let easy = a
        .iter()
        .rev()
        .zip(b.iter().rev())
        .all(|(&a, &b)| a.to_digit(10).unwrap() + b.to_digit(10).unwrap() < 10);

    println!("{}", if easy { "Easy" } else { "Hard" });
}
