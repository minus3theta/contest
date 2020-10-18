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
        q: usize,
        qs: [(i32, Usize1); q],
    }
    let mut h = n - 2;
    let mut w = n - 2;
    let mut x_len = vec![n - 2; n - 2];
    let mut y_len = vec![n - 2; n - 2];
    let mut ans = 0i64;
    for &query in &qs {
        match query {
            (1, x) => {
                let x = x - 1;
                if x < w {
                    ans += h as i64;
                    for i in x..w {
                        x_len[i] = h;
                    }
                    w = x;
                } else {
                    ans += x_len[x] as i64;
                }
            }
            (_, y) => {
                let y: usize = y - 1;
                if y < h {
                    ans += w as i64;
                    for i in y..h {
                        y_len[i] = w;
                    }
                    h = y;
                } else {
                    ans += y_len[y] as i64;
                }
            }
        }
    }

    println!("{}", (n as i64 - 2).pow(2) - ans);
}
