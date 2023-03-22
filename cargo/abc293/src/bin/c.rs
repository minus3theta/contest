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
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    }

    println!("{}", dfs(&a, 0, 0, Default::default()));
}

fn dfs(a: &[Vec<i64>], i: usize, j: usize, mut hist: HashSet<i64>) -> i64 {
    if !hist.insert(a[i][j]) {
        return 0;
    }
    if i == a.len() - 1 && j == a[0].len() - 1 {
        return 1;
    }
    let mut ret = 0;
    if i + 1 < a.len() {
        ret += dfs(a, i + 1, j, hist.clone());
    }
    if j + 1 < a[0].len() {
        ret += dfs(a, i, j + 1, hist);
    }
    ret
}
