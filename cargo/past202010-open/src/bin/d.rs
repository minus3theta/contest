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
        _n: usize,
        s: Chars,
    }
    let min_x = s.iter().take_while(|&&c| c == '.').count();
    let min_y = s.iter().rev().take_while(|&&c| c == '.').count();
    let min_sum = s.split(|&c| c == '#').map(|s| s.len()).max().unwrap();
    let x = min_x;
    let y = cmp::max(min_sum - x, min_y);

    println!("{} {}", x, y);
}
