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

fn solve(s: &str) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' | '[' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    stack.is_empty()
}

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!("{}", if solve(&s) { "Yes" } else { "No" });
}
