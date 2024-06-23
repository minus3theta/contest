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
        mut m: i32,
        hh: [i32; n],
    }
    let mut ans = 0;
    for &h in &hh {
        if h > m {
            break;
        }
        ans += 1;
        m -= h;
    }

    println!("{}", ans);
}
