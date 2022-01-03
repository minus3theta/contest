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
        es: [[Usize1; 2]; n-1],
    }
    let mut count = vec![0; n];
    for v in es.into_iter().flat_map(|e| e.into_iter()) {
        count[v] += 1;
    }
    let ans = *count.iter().max().unwrap() == n - 1;

    println!("{}", if ans { "Yes" } else { "No" });
}
