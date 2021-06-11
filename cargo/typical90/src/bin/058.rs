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

const MOD: usize = 100_000;

fn next(x: usize) -> usize {
    let mut y = 0;
    let mut temp = x;
    while temp > 0 {
        y += temp % 10;
        temp /= 10;
    }
    (x + y) % MOD
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut x = n;
    let mut appear = BTreeMap::new();
    let mut seq = vec![];
    let back = loop {
        if let Some(&pos) = appear.get(&x) {
            break pos;
        }
        appear.insert(x, seq.len());
        seq.push(x);
        x = next(x);
    };
    let index = if k < back {
        k
    } else {
        back + (k - back) % (seq.len() - back)
    };
    println!("{}", seq[index]);
}
