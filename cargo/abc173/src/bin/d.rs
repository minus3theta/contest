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
        mut aa: [i64; n],
    }
    aa.sort_by_key(|&a| cmp::Reverse(a));
    let mut ans = 0;
    let mut count = n - 1;
    for (i, &a) in aa.iter().enumerate() {
        if let Some(c) = count.checked_sub(1) {
            ans += a;
            count = c;
        }
        if i != 0 {
            if let Some(c) = count.checked_sub(1) {
                ans += a;
                count = c;
            }
        }
    }

    println!("{}", ans);
}
