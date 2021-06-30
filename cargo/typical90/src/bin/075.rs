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
    let mut factor = 0;
    for i in 2.. {
        if i * i > n {
            break;
        }
        while n % i == 0 {
            factor += 1;
            n /= i;
        }
    }
    if n > 1 {
        factor += 1;
    }
    let mut count = 0;
    while factor > 1 {
        count += 1;
        factor = factor.div_ceil(&2);
    }

    println!("{}", count);
}
