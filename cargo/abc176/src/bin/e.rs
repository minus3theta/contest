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
        m: usize,
        bs: [(Usize1, Usize1); m],
    }
    let mut rows = vec![0; h];
    let mut cols = vec![0; w];
    for &(r, c) in &bs {
        rows[r] += 1;
        cols[c] += 1;
    }
    let row_max = *rows.iter().max().unwrap();
    let col_max = *cols.iter().max().unwrap();
    let row_argmax = rows
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if p == row_max { Some(i) } else { None })
        .collect_vec();
    let col_argmax = cols
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if p == col_max { Some(i) } else { None })
        .collect_vec();
    let bs: BTreeSet<_> = bs.into_iter().collect();
    let dup = row_argmax
        .iter()
        .cartesian_product(&col_argmax)
        .all(|(&r, &c)| bs.contains(&(r, c)));
    let ans = row_max + col_max - if dup { 1 } else { 0 };

    println!("{}", ans);
}
