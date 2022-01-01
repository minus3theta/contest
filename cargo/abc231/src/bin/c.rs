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
        q: usize,
        mut a: [i64; n],
        x: [i64; q],
    }
    a.sort_unstable();
    for x in x {
        let lb = lower_bound(&a, &x);
        println!("{}", n - lb);
    }
}
