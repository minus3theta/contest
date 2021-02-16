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
        m: usize,
        mut h: [i64; n],
        mut w: [i64; m],
    }
    h.sort();
    w.sort();
    let lower = h.iter().tuples().map(|(&x, &y)| y - x).collect_vec();
    let upper = h
        .iter()
        .skip(1)
        .tuples()
        .map(|(&x, &y)| y - x)
        .collect_vec();
    let mut lower_sum = vec![0];
    for &l in &lower {
        let last = *lower_sum.last().unwrap();
        lower_sum.push(last + l);
    }
    let mut upper_sum = vec![0];
    for &u in upper.iter().rev() {
        let last = *upper_sum.last().unwrap();
        upper_sum.push(last + u);
    }
    let mut ans = 1 << 60;
    let mut ub_idx = 1;
    for &x in &w {
        while {
            let ub = *h.get(ub_idx).unwrap_or(&(1 << 60));
            x > ub
        } {
            ub_idx += 2;
        }
        let target = ub_idx - 1;
        let cost = lower_sum[target / 2] + (x - h[target]).abs() + upper_sum[n / 2 - target / 2];
        ans = cmp::min(ans, cost);
    }
    println!("{}", ans);
}
