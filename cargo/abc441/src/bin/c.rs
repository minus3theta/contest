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
        k: usize,
        x: i64,
        mut aa: [i64; n],
    }
    aa.sort();
    let mut sum = 0;
    let mut count = (n - k) as isize;
    for &a in aa.iter().take(k).rev() {
        if sum >= x {
            break;
        }
        sum += a;
        count += 1;
    }

    println!("{}", if sum >= x { count } else { -1 });
}
