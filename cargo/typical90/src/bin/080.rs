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
        d: u32,
        a: [u64; n],
    }
    let mut ans = 0i64;
    for s in 0u32..1 << n {
        let mut accum = 0;
        for (i, &x) in a.iter().enumerate() {
            if s >> i & 1 == 0 {
                continue;
            }
            accum |= x;
        }
        if s.count_ones() % 2 == 0 {
            ans += 1 << (d - accum.count_ones());
        } else {
            ans -= 1 << (d - accum.count_ones());
        }
    }

    println!("{}", ans);
}
