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

fn matches(x: char, y: char) -> bool {
    match (x, y) {
        ('?', _) | (_, '?') => true,
        _ => x == y,
    }
}

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut head_match = 0;
    for (&a, &b) in s.iter().zip(&t) {
        if matches(a, b) {
            head_match += 1;
        } else {
            break;
        }
    }
    let mut tail_match = t.len();
    for (&a, &b) in s.iter().rev().zip(t.iter().rev()) {
        if matches(a, b) {
            tail_match -= 1;
        } else {
            break;
        }
    }
    for i in 0..=t.len() {
        println!(
            "{}",
            if i <= head_match && tail_match <= i {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
