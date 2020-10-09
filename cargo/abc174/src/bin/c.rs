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

fn solve(k: usize) -> i64 {
    let mut visited = vec![false; k];
    let mut current = 7 % k;
    for i in 1.. {
        if current == 0 {
            return i;
        }
        if visited[current] {
            return -1;
        }
        visited[current] = true;
        current = (current * 10 + 7) % k;
    }
    unreachable!()
}

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    println!("{}", solve(k));
}
