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

pub fn compress<T>(array: &[T]) -> std::collections::BTreeMap<T, usize>
where
    T: Clone + PartialEq + Ord,
{
    let mut array = array.to_vec();
    array.sort();
    array.dedup();
    array.into_iter().enumerate().map(|(i, a)| (a, i)).collect()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        user: [(i64, i64); n],
    }
    let mut event = user
        .into_iter()
        .flat_map(|(a, b)| vec![(a, 1), (a + b, std::usize::MAX)].into_iter())
        .collect_vec();
    event.sort();
    let mut count = 0usize;
    let mut ans = vec![0; n + 1];
    for i in 0..event.len() - 1 {
        count = count.wrapping_add(event[i].1);
        ans[count] += event[i + 1].0 - event[i].0;
    }
    for a in ans.into_iter().skip(1) {
        print!("{} ", a);
    }
    println!();
}
