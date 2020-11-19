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
        s: (f64, f64),
        g: (f64, f64),
    }
    let dy = g.1 + s.1;
    let dx = g.0 - s.0;
    let ans = s.0 + dx * s.1 / dy;

    println!("{}", ans);
}
