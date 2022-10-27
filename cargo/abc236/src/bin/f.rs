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
        c: [i64; (1<<n)-1],
    }
    let mut c = c.into_iter().zip(1..).collect_vec();
    c.sort();
    let mut cost = 0;
    let mut basis: Vec<i32> = vec![];
    for (x, mut a) in c {
        for &b in &basis {
            if (b & -b) & a != 0 {
                a ^= b;
            }
        }
        if a != 0 {
            for b in &mut basis {
                if (a & -a) & *b != 0 {
                    *b ^= a;
                }
            }
            basis.push(a);
            cost += x;
        }
    }

    println!("{}", cost);
}
