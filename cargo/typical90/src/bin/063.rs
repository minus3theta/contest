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

fn same(p: &[Vec<i32>], pat: i32) -> u32 {
    let mut popl = BTreeMap::new();
    for col in 0..p[0].len() {
        let mut v = vec![];
        for row in 0..p.len() {
            if pat >> row & 1 != 0 {
                v.push(p[row][col]);
            }
        }
        if v.iter().all_equal() {
            *popl.entry(*v.first().unwrap()).or_insert(0) += 1;
        }
    }
    *popl.values().max().unwrap_or(&0)
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[i32; w]; h],
    }
    let mut ans = 0;
    for pat in 1i32..1 << h {
        ans = ans.max(pat.count_ones() * same(&p, pat));
    }

    println!("{}", ans);
}
