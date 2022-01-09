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

pub fn lower_bound<T: std::cmp::PartialOrd>(arr: &[T], value: &T) -> usize {
    let mut l = -1;
    let mut r = arr.len() as isize;
    while l + 1 != r {
        let m = (l + r) / 2;
        if &arr[m as usize] < value {
            l = m;
        } else {
            r = m;
        }
    }
    r as usize
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        x: i64,
    }
    let sum = a.iter().sum::<i64>();
    let mut accum = vec![0; n + 1];
    for (i, &a) in a.iter().enumerate() {
        accum[i + 1] = accum[i] + a;
    }
    let x = x + 1;
    let cycle = x / sum;
    let rem = lower_bound(&accum, &(x - cycle * sum));

    println!("{}", cycle * n as i64 + rem as i64);
}
