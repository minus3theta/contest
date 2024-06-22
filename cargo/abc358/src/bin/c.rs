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
        m: u32,
        ss: [Chars; n],
    }
    let ss = ss
        .into_iter()
        .map(|s| {
            s.into_iter()
                .enumerate()
                .map(|(i, c)| if c == 'o' { 1u32 << i } else { 0 })
                .sum::<u32>()
        })
        .collect_vec();
    let ans = (0..(1u32 << n))
        .map(|set| {
            let mut count = 0usize;
            let mut cover = 0;
            for (i, &s) in ss.iter().enumerate() {
                if (set >> i) & 1 != 0 {
                    count += 1;
                    cover |= s;
                }
            }
            if cover == (1 << m) - 1 {
                count
            } else {
                usize::MAX
            }
        })
        .min()
        .unwrap();

    println!("{}", ans);
}
