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
        k: usize,
        s: [Chars; n],
    }
    let popl = s
        .iter()
        .map(|v| {
            let mut p = vec![false; 26];
            for &c in v {
                p[(c as u8 - b'a') as usize] = true;
            }
            p
        })
        .collect_vec();
    let mut ans = 0;
    for pat in 0..(1 << n) {
        let ret = (0..26)
            .filter(|&c| {
                popl.iter()
                    .enumerate()
                    .filter(|(i, p)| (pat >> i) & 1 != 0 && p[c])
                    .count()
                    == k
            })
            .count();
        ans = ans.max(ret);
    }

    println!("{}", ans);
}
