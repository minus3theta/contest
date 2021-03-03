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

const INF: i64 = 1 << 60;

#[fastout]
fn main() {
    input! {
        n: usize,
        shop: [(i64, i64, i64); n],
    }
    let mut ans = INF;
    for &(a, p, x) in &shop {
        if a < x {
            ans = cmp::min(ans, p);
        }
    }
    println!("{}", if ans == INF { -1 } else { ans });
}
