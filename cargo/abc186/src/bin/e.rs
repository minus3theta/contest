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

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let q = a / b;
        let (d, x, y) = ext_gcd(b, a - q * b);
        (d, y, x - q * y)
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
        q: [(i64, i64, i64); t],
    }
    for &(n, s, k) in &q {
        let u = n - s;
        let (g, x, _) = ext_gcd(k, n);
        if u % g != 0 {
            println!("-1");
        } else {
            println!("{}", (x * u / g).rem_euclid(n / g));
        }
    }
}
