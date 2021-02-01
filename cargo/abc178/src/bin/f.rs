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

fn count_le(a: &[usize]) -> Vec<usize> {
    let mut count = vec![0; a.len() + 1];
    for &x in a {
        count[x] += 1;
    }
    for i in 0..a.len() {
        count[i + 1] += count[i];
    }
    count
}

fn solve(n: usize, c: &[usize], d: &[usize]) -> Option<usize> {
    let mut shift = 0;
    for i in 0..n {
        if c[i + 1] - c[i] + d[i + 1] - d[i] > n {
            return None;
        }
        shift = cmp::max(shift, c[i + 1].saturating_sub(d[i]));
    }
    Some(shift)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let c = count_le(&a);
    let d = count_le(&b);
    match solve(n, &c, &d) {
        Some(shift) => {
            println!("Yes");
            for i in 0..n {
                print!("{} ", b[(i + n - shift) % n]);
            }
        }
        None => {
            println!("No");
        }
    }
}
