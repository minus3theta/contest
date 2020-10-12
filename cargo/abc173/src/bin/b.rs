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
        ss: [String; n],
    }
    let mut popl = vec![0; 4];
    for s in &ss {
        let i = match s.as_str() {
            "AC" => 0,
            "WA" => 1,
            "TLE" => 2,
            _ => 3,
        };
        popl[i] += 1;
    }
    for (p, &s) in ["AC", "WA", "TLE", "RE"].iter().enumerate() {
        println!("{} x {}", s, popl[p]);
    }
}
