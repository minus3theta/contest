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

const INF: i64 = 1 << 30;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut forward = vec![INF; n];
    let mut backward = vec![INF; n];
    forward[0] = 0;
    for i in 0..n {
        for (d, &c) in s[i].iter().enumerate() {
            if c == '1' {
                forward[i + d + 1] = forward[i + d + 1].min(forward[i] + 1);
            }
        }
    }
    backward[n - 1] = 0;
    for i in (0..n).rev() {
        for (d, &c) in s[i].iter().enumerate() {
            if c == '1' {
                backward[i] = backward[i].min(backward[i + d + 1] + 1);
            }
        }
    }
    for k in 1..n - 1 {
        let mut ans = INF;
        for l in k.saturating_sub(m - 1)..k {
            for (d, &c) in s[l].iter().enumerate() {
                let r = l + d + 1;
                if r > k && c == '1' {
                    ans = ans.min(forward[l] + backward[r] + 1);
                }
            }
        }
        print!("{} ", if ans == INF { -1 } else { ans });
    }
    println!();
}
