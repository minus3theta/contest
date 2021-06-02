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

fn sub(p: &[(f64, f64)], c: usize) -> f64 {
    let (cx, cy) = p[c];
    let mut angles = p
        .iter()
        .enumerate()
        .filter_map(|(i, &(x, y))| {
            if i == c {
                None
            } else {
                Some(NotNan::from((y - cy).atan2(x - cx).to_degrees()))
            }
        })
        .collect_vec();
    angles.sort();
    let cycle = angles.len();
    let mut ret = 0.0f64;
    let mut j = 0;
    let diff = |i: usize, j: usize| {
        let d = angles[j] - angles[i];
        if d < 0.0.into() {
            d + 360.0
        } else {
            d
        }
    };
    let fix = |d: NotNan<f64>| {
        if d > 180.0.into() {
            NotNan::from(360.0) - d
        } else {
            d
        }
    };
    for i in 0..cycle {
        while j != i && diff(i, j) < 180.0.into() {
            ret = ret.max(fix(diff(i, j)).into());
            j = (j + 1) % cycle;
        }
        ret = ret.max(fix(diff(i, j)).into());
    }

    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(f64, f64); n],
    }
    let mut ans = 0.0f64;
    for i in 0..n {
        ans = ans.max(sub(&p, i));
    }

    println!("{:.15}", ans);
}
