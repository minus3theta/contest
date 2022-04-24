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
        a: [usize; n],
    }
    let mut popl = vec![0i64; a.iter().max().unwrap() + 1];
    for &x in &a {
        popl[x] += 1;
    }
    let ans: i64 = (1..popl.len())
        .map(|i| {
            let mut sum = 0;
            for j in 1.. {
                if j * j > i {
                    break;
                }
                if i % j == 0 {
                    let k = i / j;
                    sum += if j == k {
                        popl[i] * popl[j] * popl[k]
                    } else {
                        popl[i] * popl[j] * popl[k] * 2
                    };
                }
            }
            sum
        })
        .sum();

    println!("{}", ans);
}
