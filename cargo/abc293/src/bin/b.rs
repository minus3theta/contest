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
        a: [Usize1; n],
    }
    let mut called = vec![false; n];
    for (i, &x) in a.iter().enumerate() {
        if !called[i] {
            called[x] = true;
        }
    }
    let not_called = (0..n)
        .filter_map(|i| if !called[i] { Some(i + 1) } else { None })
        .collect_vec();
    println!("{}", not_called.len());
    for i in not_called {
        print!("{} ", i);
    }
    println!();
}
