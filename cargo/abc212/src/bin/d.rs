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
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut que = BinaryHeap::new();
    let mut offset = 0;
    for _ in 0..q {
        input! {
            t: i32,
        }
        if t == 1 {
            input! {
                x: i64,
            }
            que.push(Reverse(x - offset));
        } else if t == 2 {
            input! {
                x: i64,
            }
            offset += x;
        } else {
            let Reverse(x) = que.pop().unwrap();
            println!("{}", x + offset);
        }
    }
}
