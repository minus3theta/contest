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
        p: i64,
        q: i64,
        a: [i64; n],
    }
    let mut ans = 0;
    for i0 in 0..n {
        for i1 in 0..i0 {
            for i2 in 0..i1 {
                for i3 in 0..i2 {
                    for i4 in 0..i3 {
                        if a[i0] * a[i1] % p * a[i2] % p * a[i3] % p * a[i4] % p == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
