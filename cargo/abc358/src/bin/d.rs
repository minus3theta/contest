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

fn solve(aa: &[i64], bb: &[i64]) -> Option<i64> {
    let mut a_iter = aa.iter().peekable();
    let mut cost = 0;
    for &b in bb {
        while a_iter.peek()? < &&b {
            a_iter.next();
        }
        cost += a_iter.next()?;
    }
    Some(cost)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut aa: [i64; n],
        mut bb: [i64; m],
    }
    aa.sort();
    bb.sort();

    println!("{}", solve(&aa, &bb).unwrap_or(-1));
}
