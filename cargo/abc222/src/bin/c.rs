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

fn win(x: char, y: char) -> bool {
    match (x, y) {
        ('G', 'C') | ('C', 'P') | ('P', 'G') => true,
        _ => false,
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n],
    }
    let mut win_count = vec![0; 2 * n];
    let mut rank = (0..2 * n).collect_vec();
    for round in 0..m {
        for ij in rank.chunks(2) {
            let i = ij[0];
            let j = ij[1];
            let hi = a[i][round];
            let hj = a[j][round];
            if win(hi, hj) {
                win_count[i] += 1;
            } else if win(hj, hi) {
                win_count[j] += 1;
            }
        }
        rank = (0..2 * n)
            .sorted_by_key(|&i| (cmp::Reverse(win_count[i]), i))
            .collect();
    }
    for r in rank {
        println!("{}", r + 1);
    }
}
