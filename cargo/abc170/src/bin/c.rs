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

#[fastout]
fn main() {
    input! {
        x: i32,
        n: usize,
        p: [i32; n],
    }
    let p: std::collections::BTreeSet<_> = p.into_iter().collect();
    let mut ans = 0;
    let mut md = x;
    for i in 0..=101 {
        if !p.contains(&i) {
            if (x - i).abs() < md {
                ans = i;
                md = (x - i).abs();
            }
        }
    }

    println!("{}", ans);
}
