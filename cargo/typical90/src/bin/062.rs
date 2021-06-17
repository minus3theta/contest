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

fn solve(ball: &[(usize, usize)]) -> Option<Vec<usize>> {
    let n = ball.len();
    let mut adj = vec![vec![]; n];
    let mut available = VecDeque::new();
    let mut used = vec![false; n];
    for (i, &(a, b)) in ball.iter().enumerate() {
        adj[a].push(i);
        adj[b].push(i);
        if i == a || i == b {
            available.push_back(i);
            used[i] = true;
        }
    }
    let mut ret = vec![];
    for _ in 0..ball.len() {
        let v = available.pop_front()?;
        ret.push(v);
        for &u in &adj[v] {
            if !used[u] {
                used[u] = true;
                available.push_back(u);
            }
        }
    }

    ret.reverse();
    Some(ret)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ball: [(Usize1, Usize1); n],
    }
    if let Some(seq) = solve(&ball) {
        for &v in &seq {
            println!("{}", v + 1);
        }
    } else {
        println!("-1");
    }
}
