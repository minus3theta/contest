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
        s: Chars,
        t: Chars,
    }
    let ans = s == t
        || (|| {
            for i in 0..s.len() - 1 {
                let mut x = s.clone();
                x.swap(i, i + 1);
                if x == t {
                    return true;
                }
            }
            false
        })();

    println!("{}", if ans { "Yes" } else { "No" });
}
