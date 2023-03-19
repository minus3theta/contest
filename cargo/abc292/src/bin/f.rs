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
        a: f64,
        b: f64,
    }
    let mut l = 0.0;
    let mut r = std::f64::consts::FRAC_PI_6;
    for _ in 0..50 {
        let m = (l + r) * 0.5;
        let x = b / m.cos();
        let y = a / (m + std::f64::consts::FRAC_PI_3).sin();
        if x < y {
            l = m;
        } else {
            r = m;
        }
    }

    println!(
        "{:.15}",
        (b / l.cos()).min(a / (l + std::f64::consts::FRAC_PI_3).sin())
    );
}
