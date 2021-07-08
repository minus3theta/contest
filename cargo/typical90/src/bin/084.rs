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
        n: i64,
        s: Chars,
    }
    let mut ans = n * (n - 1) / 2;
    let mut len = 0;
    let mut current = 'a';
    for &c in &s {
        if c == current {
            len += 1;
        } else {
            ans -= len * (len + 1) / 2;
            len = 0;
            current = c;
        }
    }
    ans -= len * (len + 1) / 2;

    println!("{}", ans);
}
