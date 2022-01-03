#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut prev = vec![None; n];
    let mut next = vec![None; n];
    for _ in 0..q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                next[x] = Some(y);
                prev[y] = Some(x);
            }
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                next[x] = None;
                prev[y] = None;
            }
            3 => {
                input! {
                    x: Usize1,
                }
                let mut head = x;
                while let Some(h) = prev[head] {
                    head = h;
                }
                let mut ans = vec![head];
                while let Some(h) = next[head] {
                    head = h;
                    ans.push(head);
                }
                print!("{}", ans.len());
                for a in ans {
                    print!(" {}", a + 1);
                }
                println!();
            }
            _ => unreachable!(),
        }
    }
}
