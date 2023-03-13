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
        m: usize,
        ss: [[Usize1]; m],
    }
    let ss = ss
        .into_iter()
        .map(|s| s.into_iter().map(|a| 1 << a).sum::<u32>())
        .collect_vec();
    let mut ans = 0;
    for p in 1..1 << m {
        let mut set = 0;
        for (i, &s) in ss.iter().enumerate() {
            if (p >> i) & 1 != 0 {
                set |= s;
            }
        }
        if set == (1 << n) - 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
