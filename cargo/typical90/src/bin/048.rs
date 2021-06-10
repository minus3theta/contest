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
        k: i32,
        score: [(i64, i64); n],
    }
    let mut que = BinaryHeap::new();
    for &(a, b) in &score {
        que.push((b, a));
    }
    let mut ans = 0;
    for _ in 0..k {
        let (b, a) = que.pop().unwrap();
        ans += b;
        if a >= 0 {
            que.push((a - b, -1));
        }
    }

    println!("{}", ans);
}
