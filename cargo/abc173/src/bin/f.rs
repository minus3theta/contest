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
        es: [(Usize1, Usize1); n - 1],
    }
    let n = n as i64;
    let mut ans = 0i64;
    for i in 0..n {
        ans += (i + 1) * (n - i);
    }
    for (u, v) in es.into_iter() {
        let (u, v) = if u < v {
            (u as i64, v as i64)
        } else {
            (v as i64, u as i64)
        };
        ans -= (u + 1) * (n - v);
    }

    println!("{}", ans);
}
