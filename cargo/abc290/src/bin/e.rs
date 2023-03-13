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
        a: [Usize1; n],
    }
    let mut index = HashMap::new();
    for (i, x) in a.into_iter().enumerate() {
        index.entry(x).or_insert_with(Vec::new).push(i);
    }
    let mut ans = (1..=n).map(|i| (n + 1 - i) * (i / 2)).sum::<usize>();
    for v in index.values() {
        let mut l = 0;
        let mut r = v.len() - 1;
        while l < r {
            if v[l] + 1 < (n - v[r]) {
                ans -= (r - l) * (v[l] + 1);
                l += 1;
            } else {
                ans -= (r - l) * (n - v[r]);
                r -= 1;
            }
        }
    }

    println!("{}", ans);
}
