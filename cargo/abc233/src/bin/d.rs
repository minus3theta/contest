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
        k: i64,
        a: [i64; n],
    }
    let mut map: HashMap<i64, i64> = Some((0, 1)).into_iter().collect();
    let mut sum = 0;
    let mut ans = 0;
    for &a in &a {
        sum += a;
        ans += map.get(&(sum - k)).unwrap_or(&0);
        *map.entry(sum).or_insert(0) += 1;
    }

    println!("{}", ans);
}
