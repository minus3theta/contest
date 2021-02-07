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

fn solve(x: i64, y: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if let Some(&s) = memo.get(&y) {
        return s;
    }
    let z = (x - y).abs();
    let step = if y == 1 {
        z
    } else if y % 2 == 1 {
        *[
            z,
            solve(x, (y + 1) / 2, memo) + 2,
            solve(x, (y - 1) / 2, memo) + 2,
        ]
        .iter()
        .min()
        .unwrap()
    } else {
        cmp::min(z, solve(x, y / 2, memo) + 1)
    };
    memo.insert(y, step);
    step
}

#[fastout]
fn main() {
    input! {
        x: i64,
        y: i64,
    }
    let mut memo = HashMap::new();
    let ans = solve(x, y, &mut memo);
    println!("{}", ans);
}
