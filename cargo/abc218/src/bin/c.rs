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

fn trim(shape: &[Vec<char>]) -> Vec<Vec<bool>> {
    let mut r_min = shape.len();
    let mut r_max = 0;
    let mut c_min = shape.len();
    let mut c_max = 0;
    for (r, row) in shape.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '#' {
                r_min = r_min.min(r);
                r_max = r_max.max(r);
                c_min = c_min.min(c);
                c_max = c_max.max(c);
            }
        }
    }
    (r_min..=r_max)
        .map(|r| (c_min..=c_max).map(move |c| shape[r][c] == '#').collect())
        .collect()
}

fn rotate(shape: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let h = shape.len();
    let w = shape[0].len();
    (0..w)
        .rev()
        .map(|r| (0..h).map(move |c| shape[c][r]).collect())
        .collect()
}

fn solve(s: &[Vec<char>], t: &[Vec<char>]) -> bool {
    let mut s = trim(s);
    let t = trim(t);
    for _ in 0..4 {
        if s == t {
            return true;
        }
        s = rotate(&s);
    }
    false
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    println!("{}", if solve(&s, &t) { "Yes" } else { "No" });
}
