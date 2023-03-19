#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
#[allow(unused_imports)]
use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        ev: [(u8, Usize1); q],
    }
    let mut yellow = vec![0; n];
    for &(t, x) in &ev {
        match t {
            1 => yellow[x] += 1,
            2 => yellow[x] += 2,
            _ => {
                println!("{}", if yellow[x] >= 2 { "Yes" } else { "No" });
            }
        }
    }
}
