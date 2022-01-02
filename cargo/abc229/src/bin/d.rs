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
        mut k: usize,
    }
    let mut l = 0;
    let mut ans = 0;
    for (r, &c) in s.iter().enumerate() {
        if c == '.' {
            if k == 0 {
                while s[l] == 'X' {
                    l += 1;
                }
                l += 1;
            } else {
                k -= 1;
            }
        }
        ans = ans.max(r - l + 1);
    }

    println!("{}", ans);
}
