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

enum Ret {
    InLoop(usize, usize),
    NotLoop(usize),
}

impl Ret {
    fn reachable(&self) -> usize {
        match *self {
            Ret::InLoop(_, s) => s,
            Ret::NotLoop(s) => s,
        }
    }
}

fn dfs(
    v: usize,
    depth: usize,
    adj: &[usize],
    forward: &mut [Option<usize>],
    backward: &mut [Option<usize>],
) -> Ret {
    if let &Some(reachable) = &backward[v] {
        return Ret::NotLoop(reachable);
    }
    if let &Some(d) = &forward[v] {
        let loop_size = depth - d;
        backward[v] = Some(loop_size);
        return Ret::InLoop(v, loop_size);
    }
    forward[v] = Some(depth);
    match dfs(adj[v], depth + 1, adj, forward, backward) {
        Ret::InLoop(begin, size) => {
            backward[v] = Some(size);
            if v == begin {
                return Ret::NotLoop(size);
            }
            Ret::InLoop(begin, size)
        }
        Ret::NotLoop(reachable) => {
            backward[v] = Some(reachable + 1);
            Ret::NotLoop(reachable + 1)
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }
    let mut sum = 0usize;
    let mut forward = vec![None; n];
    let mut backward = vec![None; n];
    for i in 0..n {
        sum += dfs(i, 0, &aa, &mut forward, &mut backward).reachable();
    }

    println!("{}", sum);
}
