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
    for i in 0..n {
        let mut cycle = 1;
        let mut current = a[i];
        while current != i {
            cycle += 1;
            current = a[current];
        }
        print!("{} ", cycle);
    }
}
