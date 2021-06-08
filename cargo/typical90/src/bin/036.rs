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

fn max_diff(x: i64, range: (i64, i64)) -> i64 {
    (x - range.0).abs().max((x - range.1).abs())
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        qs: [Usize1; q],
    }
    let ab = xy.into_iter().map(|(x, y)| (x + y, x - y)).collect_vec();
    let a_range = ab.iter().map(|&(a, _)| a).minmax().into_option().unwrap();
    let b_range = ab.iter().map(|&(_, b)| b).minmax().into_option().unwrap();
    for &id in &qs {
        let (a, b) = ab[id];
        println!("{}", max_diff(a, a_range).max(max_diff(b, b_range)));
    }
}
