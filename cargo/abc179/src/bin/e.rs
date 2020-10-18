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
        x: i64,
        m: i64,
    }
    let mut current: i64 = x;
    let mut dist = vec![0; m as usize];
    let mut d = 1;
    let mut sum = vec![0; m as usize + 1];
    let (head, cycle) = loop {
        let cd = dist[current as usize];
        if cd != 0 {
            break (cd - 1, d - cd);
        }
        sum[d] = sum[d - 1] + current;
        dist[current as usize] = d;
        current = current * current % m;
        d += 1;
    };
    // dbg!((head, cycle));
    // dbg!(&sum);
    let ans = if n <= head {
        sum[n]
    } else {
        let rest = n - head;
        let quo = rest / cycle;
        let rem = rest % cycle;
        // dbg!((quo, rem));
        sum[head + rem] + (sum[head + cycle] - sum[head]) * quo as i64
    };

    println!("{}", ans);
}
