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

const MAX: i64 = 10000;

#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }
    let mut ans = 1 << 60;
    for x in 0..MAX {
        let s = a * x;
        if s > n {
            break;
        }
        for y in 0..MAX - x {
            let s = s + b * y;
            if s > n {
                break;
            }
            let r = n - s;
            if r % c == 0 {
                ans = ans.min(x + y + r / c);
            }
        }
    }

    println!("{}", ans);
}
