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
        m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut less = vec![0; n];
    for &(mut a, mut b) in &es {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        less[b] += 1;
    }

    println!("{}", less.iter().filter(|&&c| c == 1).count());
}
