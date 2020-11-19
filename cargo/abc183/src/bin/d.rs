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
        w: i64,
        us: [(usize, usize, i64); n],
    }
    let max_t: usize = us.iter().map(|&(_, t, _)| t).max().unwrap();
    let mut used = vec![0i64; max_t + 1];
    for &(s, t, p) in &us {
        used[s] += p;
        used[t] -= p;
    }
    let mut current = 0;
    for &u in &used {
        current += u;
        if current > w {
            break;
        }
    }

    println!("{}", if current > w { "No" } else { "Yes" });
}
