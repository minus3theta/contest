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
        l: i64,
        q: usize,
        query: [(i32, i64); q],
    }
    let mut cut = BTreeSet::new();
    cut.insert(0);
    cut.insert(l);
    for &(c, x) in &query {
        if c == 1 {
            cut.insert(x);
        } else {
            let lb = *cut.range(..x).next_back().unwrap();
            let ub = *cut.range(x..).next().unwrap();
            println!("{}", ub - lb);
        }
    }
}
