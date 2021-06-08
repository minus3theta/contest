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
fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let g = gcd(a, b);
    let l = b.saturating_mul(a / g);
    if l > 1_000_000_000_000_000_000 {
        println!("Large");
    } else {
        println!("{}", l);
    }
}
