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

fn solve_2(x: i64, y: i64, a: i64, b: i64) -> bool {
    fn sub(x: i64, y: i64, a: i64, b: i64) -> bool {
        let used = a.div_ceil(&x);
        let rest = y - used;
        x * rest >= b
    }
    sub(x, y, a, b) || sub(y, x, a, b)
}

fn solve(x: i64, y: i64, rect: &[i64]) -> bool {
    fn sub1(x: i64, y: i64, a: i64, b: i64, c: i64) -> bool {
        fn sub2(x: i64, y: i64, a: i64, b: i64, c: i64) -> bool {
            let used = a.div_ceil(&x);
            let rest = y - used;
            rest > 0 && solve_2(x, rest, b, c)
        }
        sub2(x, y, a, b, c) || sub2(y, x, a, b, c)
    }
    sub1(x, y, rect[0], rect[1], rect[2])
        || sub1(x, y, rect[1], rect[0], rect[2])
        || sub1(x, y, rect[2], rect[0], rect[1])
}

#[fastout]
fn main() {
    input! {
        x: i64,
        y: i64,
        rect: [i64; 3],
    }

    println!("{}", if solve(x, y, &rect) { "Yes" } else { "No" });
}
