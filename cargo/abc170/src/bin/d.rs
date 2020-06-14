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

use std::collections::{BTreeMap, BTreeSet};
fn divisor(n: i64) -> BTreeSet<i64> {
    let mut i = 1;
    let mut div = BTreeSet::new();
    while i * i <= n {
        if n % i == 0 {
            div.insert(i);
            if n / i != i {
                div.insert(n / i);
            }
        }
        i += 1;
    }
    div
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    let mut popl = BTreeMap::new();
    for &x in &a {
        *popl.entry(x).or_insert(0) += 1;
    }
    let mut set = BTreeSet::new();
    let mut ans = 0;
    for &x in &a {
        if popl[&x] > 1 {
            set.insert(x);
            continue;
        }
        let mut ok = true;
        for &d in &divisor(x) {
            if set.contains(&d) {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
        set.insert(x);
    }

    println!("{}", ans);
}
