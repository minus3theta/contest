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
        m: usize,
        a: [usize; m],
    }
    let a = a.into_iter().collect::<HashSet<_>>();
    let mut stack = Vec::new();
    for i in 1..=n {
        stack.push(i);
        if !a.contains(&i) {
            while let Some(x) = stack.pop() {
                print!("{} ", x);
            }
        }
    }

    println!();
}
