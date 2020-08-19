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
        mut ls: [i64; n],
    }
    ls.sort();
    let mut ans = 0;
    for (&x, &y, &z) in ls.iter().tuple_combinations() {
        if x == y || y == z {
            continue;
        }
        if x + y <= z {
            continue;
        }
        ans += 1;
    }

    println!("{}", ans);
}
