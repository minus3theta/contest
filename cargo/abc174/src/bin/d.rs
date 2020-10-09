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
        n: usize,
        s: Chars,
    }
    let mut ans = n;
    let mut w2r = 0;
    let mut r2w = s.iter().filter(|&&c| c == 'R').count();
    for i in 0..=n {
        ans = cmp::min(ans, cmp::max(w2r, r2w));
        if i == n {
            break;
        }
        if s[i] == 'W' {
            w2r += 1;
        } else {
            r2w -= 1;
        }
    }

    println!("{}", ans);
}
