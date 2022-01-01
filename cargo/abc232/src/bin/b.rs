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

fn to_int(s: &[char]) -> Vec<u32> {
    s.iter().map(|&c| c as u32 - 'a' as u32).collect()
}

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let s = to_int(&s);
    let t = to_int(&t);
    let diff = (t[0] + 26 - s[0]) % 26;

    println!(
        "{}",
        if s.iter()
            .map(|c| (c + diff) % 26)
            .zip(&t)
            .all(|(a, &b)| a == b)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
