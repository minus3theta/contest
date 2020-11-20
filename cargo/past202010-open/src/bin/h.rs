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
        k: usize,
        fld: [Chars; n],
    }

    println!("{}", max_size(n, m, k, &fld));
}

fn max_size(n: usize, m: usize, k: usize, fld: &Vec<Vec<char>>) -> usize {
    for size in (1..=cmp::min(n, m)).rev() {
        for up in 0..=n - size {
            for left in 0..=m - size {
                if ok(k, fld, up, left, size) {
                    return size;
                }
            }
        }
    }
    unreachable!();
}

fn ok(k: usize, fld: &Vec<Vec<char>>, up: usize, left: usize, size: usize) -> bool {
    let mut popl = vec![0; 10];
    for r in up..up + size {
        for c in left..left + size {
            popl[fld[r][c] as usize - '0' as usize] += 1;
        }
    }
    let max = popl.into_iter().max().unwrap();
    max + k >= size * size
}
