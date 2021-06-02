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
        s: Chars,
    }

    let mut next = vec![vec![0; 26]; n];
    let mut first = vec![2 * n; 26];
    for (i, &c) in s.iter().enumerate().rev() {
        let c = c as usize - 'a' as usize;
        first[c] = i;
        for j in 0..26 {
            next[i][j] = first[j];
        }
    }
    let mut p = 0;
    let mut ans = String::new();
    while ans.len() < k {
        for c in 0..26 {
            if next[p][c] + k - ans.len() - 1 < n {
                ans.push((c as u8 + 'a' as u8) as char);
                p = next[p][c] + 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
