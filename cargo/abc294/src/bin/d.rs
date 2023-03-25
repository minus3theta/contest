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
        _n: usize,
        q: usize,
    }
    let mut que = BTreeSet::new();
    let mut latest = 1;
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                que.insert(latest);
                latest += 1;
            }
            2 => {
                input! {
                    x: usize,
                }
                que.remove(&x);
            }
            3 => {
                let c = *que.iter().next().unwrap();
                println!("{}", c);
            }
            _ => unreachable!(),
        }
    }
}
