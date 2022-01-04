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
        line: [(f64, f64); n],
    }
    let total_time: f64 = line.iter().map(|&(a, b)| a / b).sum();
    let mut time = total_time / 2.0;
    let mut len = 0.0;
    for &(a, b) in &line {
        let t = a / b;
        if time < t {
            len += time * b;
            break;
        }
        time -= t;
        len += a;
    }

    println!("{:.8}", len);
}
