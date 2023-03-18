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

fn solve(s: &str) -> bool {
    let mut hist = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for c in s.chars() {
        hist.insert((x, y));
        match c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => unreachable!(),
        }
        if hist.contains(&(x, y)) {
            return true;
        }
    }
    false
}

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }

    println!("{}", if solve(&s) { "Yes" } else { "No" });
}
