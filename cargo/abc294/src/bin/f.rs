#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
use ordered_float::NotNan;
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
        m: usize,
        k: usize,
        ab: [(f64, f64); n],
        cd: [(f64, f64); m],
    }
    let mut l = 0.0;
    let mut r = 100.0;
    for _ in 0..100 {
        let m = (l + r) / 2.0;
        if count(&ab, &cd, m) >= k {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{:.15}", l);
}

fn sort(v: &[(f64, f64)], x: f64) -> Vec<NotNan<f64>> {
    let mut s: Vec<_> = v
        .iter()
        .map(|&(a, b)| ((100.0 - x) * a - b * x).into())
        .collect();
    s.sort();
    s
}

fn count(ab: &[(f64, f64)], cd: &[(f64, f64)], x: f64) -> usize {
    let ab = sort(ab, x);
    let mut cd = sort(cd, x);
    cd.reverse();
    let mut i = 0;
    let mut count = 0;
    for &a in ab.iter() {
        while i < cd.len() && a + cd[i] >= 0.0.into() {
            i += 1;
        }
        count += i;
    }
    count
}
