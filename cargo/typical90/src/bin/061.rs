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
        q: usize,
    }
    let mut deck = VecDeque::new();
    for _ in 0..q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                input! {
                    x: i64,
                }
                deck.push_front(x);
            }
            2 => {
                input! {
                    x: i64,
                }
                deck.push_back(x);
            }
            _ => {
                input! {
                    x: Usize1,
                }
                println!("{}", deck[x]);
            }
        }
    }
}
