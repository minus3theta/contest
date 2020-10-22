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
        n: i64,
    }
    let mut ans = BTreeSet::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            ans.insert(i);
            ans.insert(n / i);
        }
        i += 1;
    }

    for a in ans {
        println!("{}", a);
    }
}
