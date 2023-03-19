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

fn count_divisor(x: usize) -> i64 {
    let mut ret = 0;
    for i in 1.. {
        if i * i > x {
            break;
        }
        if x % i == 0 {
            ret += if i * i == x { 1 } else { 2 };
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let divisor = (0..=n).map(count_divisor).collect_vec();
    let mut ans = 0;
    for ab in 1..n {
        ans += divisor[ab] * divisor[n - ab];
    }

    println!("{}", ans);
}
