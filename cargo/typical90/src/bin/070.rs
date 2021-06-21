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
        ps: [(i64, i64); n],
    }
    let mut xs = ps.iter().map(|&(x, _)| x).collect_vec();
    xs.sort();
    let mut ys = ps.iter().map(|&(_, y)| y).collect_vec();
    ys.sort();
    let cx = xs[n / 2];
    let cy = ys[n / 2];
    let ans: i64 = ps
        .into_iter()
        .map(|(x, y)| (x - cx).abs() + (y - cy).abs())
        .sum();

    println!("{}", ans);
}
