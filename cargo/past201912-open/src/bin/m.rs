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
        normal: [(f64, f64); n],
        helper: [(f64, f64); m],
    }
    let mut l = 0.0;
    let mut r = 1e6;
    for _ in 0..100 {
        let mid = (l + r) / 2.0;
        let mut monsters = normal
            .iter()
            .map(|&(w, p)| cmp::Reverse(NotNan::new(p - w * mid).unwrap()))
            .collect_vec();
        let helper = helper
            .iter()
            .map(|&(w, p)| NotNan::new(p - w * mid).unwrap())
            .max()
            .unwrap();
        monsters.push(cmp::Reverse(helper));
        monsters.sort();
        let total = monsters
            .iter()
            .take(5)
            .map(|&cmp::Reverse(p)| p.into_inner())
            .sum::<f64>();
        if total >= 0.0 {
            l = mid;
        } else {
            r = mid;
        };
    }

    println!("{}", l);
}
