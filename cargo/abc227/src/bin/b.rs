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

fn calc(a: i64, b: i64) -> i64 {
    4 * a * b + 3 * a + 3 * b
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [i64; n],
    }
    let mut candidate = HashSet::new();
    for a in 1.. {
        if calc(a, 1) > 1000 {
            break;
        }
        for b in 1.. {
            let c = calc(a, b);
            if c > 1000 {
                break;
            }
            candidate.insert(c);
        }
    }

    println!("{}", s.iter().filter(|&s| !candidate.contains(s)).count());
}
