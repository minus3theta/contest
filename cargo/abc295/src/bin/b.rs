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

fn dist(i: usize, j: usize, bi: usize, bj: usize) -> i32 {
    (bi as i32 - i as i32).abs() + (bj as i32 - j as i32).abs()
}

fn power(c: char) -> i32 {
    match c {
        '1'..='9' => c as i32 - '0' as i32,
        _ => -1,
    }
}

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    }
    let mut after = b.clone();
    for (i, row) in after.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if (0..r)
                .cartesian_product(0..c)
                .any(|(bi, bj)| dist(i, j, bi, bj) <= power(b[bi][bj]))
            {
                *cell = '.';
            }
        }
    }

    for row in &after {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
