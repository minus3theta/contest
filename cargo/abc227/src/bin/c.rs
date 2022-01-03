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
        n: i64,
    }
    let mut ans = 0;
    for a in 1.. {
        if a * a * a > n {
            break;
        }
        for b in a.. {
            if a * b * b > n {
                break;
            }
            ans += n / (a * b) - b + 1;
        }
    }

    println!("{}", ans);
}
