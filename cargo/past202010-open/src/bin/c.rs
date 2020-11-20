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
        fld: [Chars; n],
    }
    let ds = [std::usize::MAX, 0, 1];
    for r in 0..n {
        for c in 0..m {
            let mut count = 0;
            for &dr in &ds {
                for &dc in &ds {
                    let nr = r.wrapping_add(dr);
                    let nc = c.wrapping_add(dc);
                    if nr < n && nc < m {
                        count += if fld[nr][nc] == '#' { 1 } else { 0 };
                    }
                }
            }
            print!("{}", count);
        }
        println!();
    }
}
