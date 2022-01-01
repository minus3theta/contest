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
        _n: i64,
        a: i64,
        b: i64,
        pq: (i64,i64),
        rs: (i64,i64),
    }

    for i in pq.0..=pq.1 {
        for j in rs.0..=rs.1 {
            print!(
                "{}",
                if i - a == j - b || i - a == b - j {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}
