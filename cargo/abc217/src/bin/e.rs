#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
// use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

// #[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut front = BinaryHeap::new();
    let mut back = VecDeque::new();
    for _ in 0..q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                input! {
                    x: i64,
                }
                back.push_back(x);
            }
            2 => {
                let x = if let Some(cmp::Reverse(x)) = front.pop() {
                    x
                } else {
                    back.pop_front().unwrap()
                };
                println!("{}", x);
            }
            3 => {
                front.extend(back.drain(..).map(cmp::Reverse));
            }
            _ => unreachable!(),
        }
    }
}
