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
        aa: [i64; n],
        bb: [i64; m],
    }
    let aa = aa.into_iter().collect::<BTreeSet<_>>();
    let ans = bb
        .iter()
        .flat_map(|&b| {
            aa.range(..=b)
                .next_back()
                .map(|&a| b - a)
                .into_iter()
                .chain(aa.range(b..).next().map(|&a| a - b))
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
