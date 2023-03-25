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
        a: [i32; n],
        b: [i32; m],
    }
    let mut ans_a = Vec::new();
    let mut ans_b = Vec::new();
    let mut a = a.into_iter().peekable();
    let mut b = b.into_iter().peekable();
    for i in 1..=n + m {
        match (a.peek(), b.peek()) {
            (Some(&x), Some(&y)) => {
                if x < y {
                    a.next();
                    ans_a.push(i);
                } else {
                    b.next();
                    ans_b.push(i);
                }
            }

            (None, None) => (),
            (None, Some(_)) => {
                b.next();
                ans_b.push(i);
            }
            (Some(_), None) => {
                a.next();
                ans_a.push(i);
            }
        }
    }
    for &i in &ans_a {
        print!("{} ", i);
    }
    println!();
    for &i in &ans_b {
        print!("{} ", i);
    }
    println!();
}
