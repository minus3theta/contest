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
        aa: [i32; 2 * n],
    }
    println!(
        "{}",
        aa.iter()
            .zip(aa.iter().skip(2))
            .filter(|(&x, &y)| x == y)
            .count()
    );
}
