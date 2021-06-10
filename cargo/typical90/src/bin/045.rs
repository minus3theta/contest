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

fn solve(ps: &[(i64, i64)], pat: i32, k: u32) -> i64 {
    let first = pat & -pat;
    let cost = |group: i32| {
        let group = group | first;
        let mut c = 0;
        for i in 0..ps.len() {
            if (group >> i) & 1 == 0 {
                continue;
            }
            for j in 0..i {
                if (group >> j) & 1 == 0 {
                    continue;
                }
                c = c.max((ps[i].0 - ps[j].0).pow(2) + (ps[i].1 - ps[j].1).pow(2));
            }
        }
        c
    };

    let free = pat & !first;
    if k == 1 {
        return cost(free);
    }
    assert_ne!(free, 0);
    let mut used = free;
    let mut ret = solve(ps, free, k - 1);
    while { used != 0 } {
        let rest = free & !used;
        if rest.count_ones() < k - 1 {
            used = (used - 1) & free;
            continue;
        }
        ret = ret.min(cost(used).max(solve(ps, rest, k - 1)));
        used = (used - 1) & free;
    }

    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u32,
        ps: [(i64, i64); n],
    }

    println!("{}", solve(&ps, (1 << n) - 1, k));
}
