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
        d: i64,
        mut lr: [(i64, i64); n],
    }
    lr.sort_by_key(|&(l, r)| (r, l));
    let mut ans = 0;
    let mut last = 0;
    for &(l, r) in &lr {
        if l <= last {
            continue;
        }
        ans += 1;
        last = r + d - 1;
    }

    println!("{}", ans);
}
