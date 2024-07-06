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
        hh: [i64; n],
    }
    let mut walls = vec![(-1, i64::MAX)];
    let mut amount = 0;
    for (i, &h) in hh.iter().enumerate() {
        let mut lowest = 0;
        while let Some(&(left, x)) = walls.last() {
            amount += (i as i64 - left) * (x.min(h) - lowest);
            lowest = x.min(h);
            if x > h {
                break;
            }
            walls.pop();
        }
        walls.push((i as i64, h));
        print!("{} ", amount + 1);
    }
}
