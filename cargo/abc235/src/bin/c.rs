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
        q: usize,
        a: [i64; n],
        query: [(i64, Usize1); q],
    }
    let mut map = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        map.entry(x).or_insert_with(|| vec![]).push(i);
    }
    for &(x, k) in &query {
        let solve = || {
            let v = map.get(&x)?;
            v.get(k)
        };
        if let Some(a) = solve() {
            println!("{}", a + 1);
        } else {
            println!("-1");
        }
    }
}
