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
        s: usize,
        ab: [(usize, usize); n],
    }
    let mut prev = vec![vec![None; s + 1]; n + 1];
    prev[0][0] = Some((-1, 0));
    for (i, &(a, b)) in ab.iter().enumerate() {
        for p in 0..=s {
            if let Some(q) = p.checked_sub(a) {
                if prev[i][q].is_some() {
                    prev[i + 1][p] = Some((0, q));
                    continue;
                }
            }
            if let Some(q) = p.checked_sub(b) {
                if prev[i][q].is_some() {
                    prev[i + 1][p] = Some((1, q));
                }
            }
        }
    }

    if prev[n][s].is_none() {
        println!("Impossible");
    } else {
        let mut list = vec![];
        let mut p = s;
        for i in (1..=n).rev() {
            let (t, q) = prev[i][p].unwrap();
            list.push(if t == 0 { 'A' } else { 'B' });
            p = q;
        }
        println!("{}", list.iter().rev().collect::<String>());
    }
}
