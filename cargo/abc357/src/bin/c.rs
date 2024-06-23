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

fn go(n: u32, top: usize, left: usize, field: &mut [Vec<bool>]) {
    if n == 0 {
        field[top][left] = true;
        return;
    }
    let size = 3_usize.pow(n - 1);
    for i in 0..3 {
        for j in 0..3 {
            if (i, j) != (1, 1) {
                go(n - 1, top + size * i, left + size * j, field);
            }
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    let size = 3_usize.pow(n);
    let mut field = vec![vec![false; size]; size];
    go(n, 0, 0, &mut field);
    for row in &field {
        for &c in row {
            print!("{}", if c { "#" } else { "." });
        }
        println!();
    }
}
