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
        a: [i32; n],
    }
    let mut offset = 0;
    let mut cuts = BTreeSet::new();
    cuts.insert(0);
    cuts.insert(360);
    for &a in &a {
        offset += a;
        offset %= 360;
        cuts.insert(offset);
    }
    let ans = cuts
        .iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .max()
        .unwrap();

    println!("{}", ans);
}
