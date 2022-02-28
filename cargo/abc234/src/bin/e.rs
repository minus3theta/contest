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
        x: i64,
    }
    let mut set = BTreeSet::<i64>::new();
    for length in 1..=18 {
        for diff in -9..=9 {
            for msb in 1..=9 {
                let lsb = msb + (length - 1) * diff;
                if lsb < 0 || lsb > 9 {
                    continue;
                }
                let mut v = 0;
                let mut d = msb;
                for _ in 0..length {
                    v *= 10;
                    v += d;
                    d += diff;
                }
                set.insert(v);
            }
        }
    }

    println!("{}", set.range(x..).next().unwrap());
}
