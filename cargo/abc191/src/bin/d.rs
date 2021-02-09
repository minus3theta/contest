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

const M: i64 = 10_000;

fn count_upper(dx2: i64, y: i64, r2: i64, lb: i64, range: i64) -> i64 {
    let in_circle = |b: i64| dx2 + (b * M - y).pow(2) <= r2;
    if !in_circle(lb) {
        return 0;
    }
    let mut i = lb;
    let mut o = range;
    while i + 1 != o {
        let m = (i + o) / 2;
        if in_circle(m) {
            i = m;
        } else {
            o = m;
        }
    }
    o - lb
}

fn count_point(a: i64, x: i64, y: i64, r: i64, range: i64) -> i64 {
    let dx2 = (a * M - x).pow(2);
    let r2 = r.pow(2);
    count_upper(dx2, y, r2, 1, range) + count_upper(dx2, M - y, r2, 1, range)
}

#[fastout]
fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64,
    }
    let x = ((x - x.floor()) * 1e4 + 0.5) as i64;
    let y = ((y - y.floor()) * 1e4 + 0.5) as i64;
    let r = (r * 1e4 + 0.5) as i64;
    let mut count = 0;
    let range = r / M + 5;
    for a in -range..=range {
        count += count_point(a, x, y, r, range);
    }
    println!("{}", count);
}
