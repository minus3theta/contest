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
        mut cs: [[Usize1]; m],
    }
    let mut top_cols = vec![vec![]; n];
    let mut que = VecDeque::new();
    for (i, c) in cs.iter_mut().enumerate() {
        c.reverse();
        let ball = *c.last().unwrap();
        top_cols[ball].push(i);
        if top_cols[ball].len() == 2 {
            que.push_back(ball);
        }
    }
    while let Some(ball) = que.pop_front() {
        let mut removed_cols = vec![];
        std::mem::swap(&mut removed_cols, &mut top_cols[ball]);
        for col in removed_cols {
            assert_eq!(cs[col].pop(), Some(ball));
            if let Some(&top) = cs[col].last() {
                top_cols[top].push(col);
                if top_cols[top].len() == 2 {
                    que.push_back(top);
                }
            }
        }
    }

    println!(
        "{}",
        if cs.iter().all(|c| c.is_empty()) {
            "Yes"
        } else {
            "No"
        }
    );
}
