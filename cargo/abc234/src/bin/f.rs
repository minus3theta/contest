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
        s: Chars,
    }
    let n = s.len();
    let mut popl = vec![0; 26];
    for c in s {
        popl[c as usize - 'a' as usize] += 1;
    }
    let mut dp = vec![vec![0; n + 1]; 26 + 1];
    dbg!(popl);

    println!("{}", 0);
}
