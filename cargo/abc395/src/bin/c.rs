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
use std::collections::btree_map::Entry;
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut ans = usize::MAX;
    let mut last_occurence = BTreeMap::new();
    for (i, &x) in a.iter().enumerate() {
        match last_occurence.entry(x) {
            Entry::Occupied(mut e) => {
                ans = ans.min(i + 1 - e.get());
                e.insert(i);
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
    }

    println!("{}", ans as isize);
}
