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

fn get_max(v: &[(usize, i64)]) -> &(usize, i64) {
    v.iter().max_by_key(|&(_, x)| x).unwrap()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; 1 << n],
    }
    let a = a.into_iter().enumerate().collect_vec();
    let (fst, snd) = a.split_at(1 << (n - 1));
    let fst_max = get_max(fst);
    let snd_max = get_max(snd);
    let ans = if fst_max.1 > snd_max.1 {
        snd_max.0
    } else {
        fst_max.0
    };
    println!("{}", ans + 1);
}
