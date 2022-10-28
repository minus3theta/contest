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
use std::iter::repeat;

fn solve(s: &[char]) -> bool {
    if s.iter().all(|&c| c == 'a') {
        return true;
    }
    let leading = s.iter().take_while(|&&c| c == 'a').count();
    let trailing = s.iter().rev().take_while(|&&c| c == 'a').count();
    if leading > trailing {
        false
    } else {
        let x = repeat('a')
            .take(trailing - leading)
            .chain(s.iter().cloned())
            .collect_vec();
        let mut y = x.clone();
        y.reverse();
        x == y
    }
}

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!("{}", if solve(&s) { "Yes" } else { "No" });
}
