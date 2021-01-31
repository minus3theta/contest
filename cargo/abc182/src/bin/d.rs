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
        a: [i64; n],
    }
    let mut ans = 0;
    let mut pos = 0;
    let mut sum = 0;
    let mut sum_max = 0;
    for &x in &a {
        sum += x;
        sum_max = cmp::max(sum_max, sum);
        ans = cmp::max(ans, pos + sum_max);
        pos += sum;
    }

    println!("{}", ans);
}
