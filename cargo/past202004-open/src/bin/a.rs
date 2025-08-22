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

fn parse(s: &str) -> i32 {
    let is_b = s.starts_with('B');
    let digit: i32 = s
        .chars()
        .filter(|&c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap();
    if is_b {
        1 - digit
    } else {
        digit
    }
}

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{}", (parse(&s) - parse(&t)).abs());
}
