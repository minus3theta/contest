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
        mut tasks: [(Usize1, i64); n],
    }
    let mut queue = BinaryHeap::new();
    tasks.sort();
    let mut sum = 0;
    let mut iter = tasks.into_iter().peekable();
    for i in 0..n {
        while let Some(&(j, t)) = iter.peek() {
            if j <= i {
                queue.push(t);
                iter.next();
            } else {
                break;
            }
        }
        if let Some(t) = queue.pop() {
            sum += t;
            println!("{}", sum);
        } else {
            unreachable!();
        }
    }
}
