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
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut aa: [i64; n],
    }
    aa.sort();
    let mut aa = aa.into_iter();
    let first = aa.next().unwrap();
    let mut queue = BinaryHeap::<(Reverse<i64>, i64, i64)>::new();
    let mut sum = 0;
    queue.push((Reverse(first), 0, first));
    for a in aa {
        let (Reverse(gain), d, x) = queue.pop().unwrap();
        sum += gain + a;
        let new_gain = ((d + 2).pow(2) - (d + 1).pow(2)) * x;
        queue.push((Reverse(new_gain), d + 1, x));
        queue.push((Reverse(3 * a), 1, a));
    }

    println!("{}", sum);
}
