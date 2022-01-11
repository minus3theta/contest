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
        p: [Usize1; n],
    }
    let mut q = vec![0; n];
    for (i, &p) in p.iter().enumerate() {
        q[p] = i + 1;
    }
    for q in q {
        print!("{} ", q);
    }
    println!();
}
