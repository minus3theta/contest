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

fn pieces(a: &[i64], min_len: i64) -> usize {
    let mut count = 0;
    let mut last_cut = 0;
    for &x in a {
        if x - last_cut >= min_len {
            count += 1;
            last_cut = x;
        }
    }

    count
}

#[fastout]
fn main() {
    input! {
        n: usize,
        len: i64,
        k: usize,
        mut a: [i64; n],
    }
    a.push(len);

    let mut l = 0;
    let mut r = len;
    assert!(pieces(&a, l) >= k + 1);
    assert!(pieces(&a, r) < k + 1);
    while l + 1 < r {
        let m = (l + r) / 2;
        if pieces(&a, m) >= k + 1 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}
