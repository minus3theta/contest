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

fn sub(p: (i64, i64), q: (i64, i64)) -> (i64, i64) {
    (p.0 - q.0, p.1 - q.1)
}

fn par(p: (i64, i64), q: (i64, i64)) -> bool {
    p.0 * q.1 - p.1 * q.0 == 0
}

#[fastout]
fn main() {
    input! {
        n: usize,
        point: [(i64, i64); n],
    }
    let ans = point
        .iter()
        .tuple_combinations()
        .filter(|(&p, &q, &r)| !par(sub(p, q), sub(p, r)))
        .count();

    println!("{}", ans);
}
