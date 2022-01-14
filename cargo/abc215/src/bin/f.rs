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

fn dist_geq(ps: &[(i64, i64)], d: i64) -> bool {
    let mut left = 0;
    let mut min = 1 << 40;
    let mut max = 0;
    for (i, &(x, y)) in ps.iter().enumerate() {
        while left < i && x - ps[left].0 >= d {
            min = min.min(ps[left].1);
            max = max.max(ps[left].1);
            left += 1;
        }
        if y - min >= d || max - y >= d {
            return true;
        }
    }
    false
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ps: [(i64, i64); n],
    }
    ps.sort();
    let mut ok = 0;
    let mut ng = 1 << 40;
    while ok + 1 != ng {
        let d = (ok + ng) / 2;
        if dist_geq(&ps, d) {
            ok = d;
        } else {
            ng = d;
        }
    }

    println!("{}", ok);
}
