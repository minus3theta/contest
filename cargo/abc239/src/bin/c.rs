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

fn dist2(p: (i64, i64), q: (i64, i64)) -> i64 {
    (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2)
}

fn solve(p: &[(i64, i64)]) -> bool {
    for x in p[0].0 - 2..=p[0].0 + 2 {
        for y in p[0].1 - 2..=p[0].1 + 2 {
            if p.iter().all(|&q| dist2(q, (x, y)) == 5) {
                return true;
            }
        }
    }
    false
}

#[fastout]
fn main() {
    input! {
        p: [(i64, i64); 2],
    }

    println!("{}", if solve(&p) { "Yes" } else { "No" });
}
