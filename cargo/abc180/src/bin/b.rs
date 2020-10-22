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
        xs: [i64; n],
    }
    let m: i64 = xs.iter().map(|&x| x.abs()).sum();
    println!("{}", m);

    let e = (xs.iter().map(|&x| x.pow(2)).sum::<i64>() as f64).sqrt();
    println!("{:.15}", e);

    let c = xs.iter().map(|&x| x.abs()).max().unwrap();
    println!("{}", c);
}
