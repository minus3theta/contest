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
        a: [i32; n],
    }
    for (&x, &y) in a.iter().tuple_windows() {
        match x.cmp(&y) {
            cmp::Ordering::Less => {
                println!("up {}", y - x);
            }
            cmp::Ordering::Equal => {
                println!("stay");
            }
            cmp::Ordering::Greater => {
                println!("down {}", x - y);
            }
        }
    }
}
