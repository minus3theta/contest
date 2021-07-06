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
        _n: usize,
        s: Chars,
    }
    let mut ans = 0i64;
    for (i, &c) in s.iter().enumerate() {
        ans += match c {
            'b' => 1,
            'c' => 2,
            _ => 0,
        } << i;
    }

    println!("{}", ans);
}
