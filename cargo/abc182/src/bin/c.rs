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
        n: Chars,
    }
    let n: Vec<_> = n.into_iter().map(|c| c as i32 - '0' as i32).collect();
    let parity = n.iter().sum::<i32>() % 3;
    let mut popl = vec![0; 3];
    for &x in &n {
        popl[x as usize % 3] += 1;
    }
    let ans = match parity {
        0 => 0,
        1 => {
            if popl[1] >= 1 {
                1
            } else if popl[2] >= 2 {
                2
            } else {
                -1
            }
        }
        _ => {
            if popl[2] >= 1 {
                1
            } else if popl[1] >= 2 {
                2
            } else {
                -1
            }
        }
    };
    let ans = if ans < n.len() as i32 { ans } else { -1 };

    println!("{}", ans);
}
