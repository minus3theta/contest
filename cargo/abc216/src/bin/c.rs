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
        mut n: i64,
    }
    let mut ans = vec![];
    while n > 0 {
        if n % 2 == 1 {
            n -= 1;
            ans.push('A');
        } else {
            n /= 2;
            ans.push('B');
        }
    }

    println!("{}", ans.iter().rev().collect::<String>());
}
