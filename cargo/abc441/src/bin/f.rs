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
        m: usize,
        items: [(usize, i64); n],
    }
    let mut max_value = vec![vec![0; m + 1]; n + 1];
    for (i, &(p, v)) in items.iter().enumerate() {
        for cost in 0..=m {
            let mut current = max_value[i][cost];
            if let Some(c) = cost.checked_sub(p) {
                current = current.max(max_value[i][c] + v);
            }
            max_value[i + 1][cost] = current;
        }
    }
    let max = *max_value[n].iter().max().unwrap();
    let mut on_route = vec![vec![false; m + 1]; n + 1];
    for (i, &v) in max_value[n].iter().enumerate() {
        if v == max {
            on_route[n][i] = true;
        }
    }
    let mut cat = vec!['X'; n];
    for i in (0..n).rev() {
        let mut can_use = false;
        let mut can_skip = false;
        for p in 0..=m {
            if !on_route[i + 1][p] {
                continue;
            }
            if max_value[i + 1][p] == max_value[i][p] {
                can_skip = true;
                on_route[i][p] = true;
            }
            if let Some(pp) = p.checked_sub(items[i].0)
                && max_value[i + 1][p] == max_value[i][pp] + items[i].1
            {
                can_use = true;
                on_route[i][pp] = true;
            }
        }
        cat[i] = match (can_use, can_skip) {
            (true, false) => 'A',
            (true, true) => 'B',
            (false, true) => 'C',
            _ => 'Z',
        };
    }
    for &c in &cat {
        print!("{c}");
    }

    println!();
}
