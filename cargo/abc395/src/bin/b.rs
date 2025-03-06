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
    }
    let mut grid = vec![vec![true; n]; n];
    for i in 0..n {
        let j = n - i;
        if i > j {
            break;
        }
        for r in i..j {
            for c in i..j {
                grid[r][c] = i % 2 == 0;
            }
        }
    }

    for row in grid {
        println!(
            "{}",
            row.into_iter()
                .map(|b| if b { '#' } else { '.' })
                .collect::<String>()
        );
    }
}
