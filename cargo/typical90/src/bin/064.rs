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
        q: usize,
        a: [i64; n],
        qs: [(Usize1, Usize1, i64); q],
    }
    let mut diff = a.iter().tuple_windows().map(|(x, y)| y - x).collect_vec();
    let mut cost: i64 = diff.iter().map(|&d| d.abs()).sum();
    for (l, r, v) in qs {
        if let Some(l) = l.checked_sub(1) {
            let old = diff[l];
            let new = old + v;
            diff[l] = new;
            cost += new.abs() - old.abs();
        }
        if r != n - 1 {
            let old = diff[r];
            let new = old - v;
            diff[r] = new;
            cost += new.abs() - old.abs();
        }
        println!("{}", cost);
    }
}
