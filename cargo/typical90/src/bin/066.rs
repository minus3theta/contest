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
        lr: [(i32, i32); n],
    }
    let mut ans = 0.0;
    for (&(li, ri), &(lj, rj)) in lr.iter().tuple_combinations() {
        let mut pat = 0;
        for i in li..=ri {
            for j in lj..=rj {
                if i > j {
                    pat += 1;
                }
            }
        }
        ans += pat as f64 / ((ri - li + 1) * (rj - lj + 1)) as f64;
    }

    println!("{:.15}", ans);
}
