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
        mut s: (i64, i64),
        mut t: (i64, i64),
    }
    let normalize = |p: &mut (i64, i64)| {
        if (p.0 + p.1) % 2 == 1 {
            p.0 -= 1;
        }
    };
    normalize(&mut s);
    normalize(&mut t);
    let dx = (s.0 - t.0).abs();
    let dy = (s.1 - t.1).abs();

    println!("{}", (dx.max(dy) + dy) / 2);
}
