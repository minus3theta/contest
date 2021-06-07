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

const SIZE: usize = 1001;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(usize, usize, usize, usize); n],
    }
    let mut fld = vec![vec![0; SIZE + 1]; SIZE + 1];

    for &(lx, ly, rx, ry) in &s {
        fld[lx][ly] += 1;
        fld[lx][ry] -= 1;
        fld[rx][ly] -= 1;
        fld[rx][ry] += 1;
    }

    for x in 0..=SIZE {
        for y in 0..SIZE {
            fld[x][y + 1] += fld[x][y];
        }
    }
    for x in 0..SIZE {
        for y in 0..=SIZE {
            fld[x + 1][y] += fld[x][y];
        }
    }
    let mut ans = vec![0; n + 1];
    for row in &fld {
        for &c in row {
            ans[c as usize] += 1;
        }
    }
    for &a in ans.iter().skip(1) {
        println!("{}", a);
    }
}
