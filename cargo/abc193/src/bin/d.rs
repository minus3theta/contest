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

type Hand = Vec<u32>;

fn parse_hand(s: &[char]) -> Hand {
    let mut hand = vec![0; 10];
    for &c in s.iter().take(4) {
        hand[c as usize - '0' as usize] += 1;
    }
    hand
}

fn score(hand: &Hand) -> i64 {
    let mut s = 0;
    for (i, &c) in hand.iter().enumerate() {
        s += i as i64 * 10i64.pow(c);
    }
    s
}

#[fastout]
fn main() {
    input! {
        k: u32,
        s: Chars,
        t: Chars,
    }
    let s = parse_hand(&s);
    let t = parse_hand(&t);
    let mut left = vec![0; 10];
    for i in 1..=9 {
        left[i] = k - s[i] - t[i];
    }
    let total_left = left.iter().sum::<u32>() as i64;
    let pattern = total_left * (total_left - 1);
    let mut s_win = 0i64;
    for i in 1..=9 {
        let mut s = s.clone();
        s[i] += 1;
        for j in 1..=9 {
            let mut t = t.clone();
            t[j] += 1;
            if score(&s) > score(&t) {
                let pat = if i == j {
                    let count = left[i] as i64;
                    count * (count - 1)
                } else {
                    let ci = left[i] as i64;
                    let cj = left[j] as i64;
                    ci * cj
                };
                s_win += pat;
            }
        }
    }
    println!("{:.15}", s_win as f64 / pattern as f64);
}
