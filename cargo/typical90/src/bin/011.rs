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
        mut tasks: [(Usize1, usize, i64); n],
    }
    tasks.sort_by_key(|&(d, _, _)| d);
    let days = tasks.last().unwrap().0 + 1;
    let mut gain = vec![vec![0; days + 1]; n + 1];

    for day in 0..=days {
        for (t, &(d, c, s)) in tasks.iter().enumerate() {
            gain[t + 1][day] = gain[t + 1][day].max(gain[t][day]);
            if day + c <= d + 1 {
                gain[t + 1][day + c] = gain[t + 1][day + c].max(gain[t][day] + s);
            }
        }
    }

    println!("{}", gain[n].iter().max().unwrap());
}
