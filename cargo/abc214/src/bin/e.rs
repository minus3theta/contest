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

fn solve(case: Vec<(i64, i64)>) -> bool {
    let mut unseen = BTreeMap::new();
    for (l, r) in case {
        unseen.entry(l).or_insert_with(|| vec![]).push(r);
    }
    let mut unseen = unseen.iter().peekable();
    let mut pos = *unseen.peek().unwrap().0;
    let mut que = BinaryHeap::new();
    loop {
        while let Some(&(&l, rs)) = unseen.peek() {
            if l > pos {
                break;
            }
            unseen.next();
            que.extend(rs.iter().map(|&r| Reverse(r)));
        }
        if let Some(Reverse(r)) = que.pop() {
            if r < pos {
                return false;
            }
            pos += 1;
        } else if let Some(&(&l, _)) = unseen.peek() {
            pos = l;
        } else {
            return true;
        }
    }
}

#[fastout]
fn main() {
    input! {
        t: usize,
        cases: [[(i64, i64)]; t],
    }
    for case in cases {
        println!("{}", if solve(case) { "Yes" } else { "No" });
    }
}
