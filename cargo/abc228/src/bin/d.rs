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

const N: i64 = 1 << 20;

#[fastout]
fn main() {
    input! {
        q: usize,
        query: [(i32, i64); q],
    }
    let mut set: BTreeSet<i64> = (0..N).collect();
    let mut value = BTreeMap::new();
    for &(t, x) in &query {
        if t == 1 {
            let h = x % N;
            let mut it = set.range(h..);
            let k = if let Some(&k) = it.next() {
                k
            } else {
                *set.iter().next().unwrap()
            };
            set.remove(&k);
            value.insert(k, x);
        } else {
            println!("{}", value.get(&(x % N)).unwrap_or(&-1));
        }
    }
}
