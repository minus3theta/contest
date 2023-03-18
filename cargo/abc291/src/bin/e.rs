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
        es: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![]; n];
    let mut deg = vec![0; n];
    for (u, v) in es {
        adj[u].push(v);
        deg[v] += 1;
    }
    if let Some(r) = solve(&adj, &mut deg) {
        println!("Yes");
        let mut ans = vec![0; n];
        for (i, &x) in r.iter().enumerate() {
            ans[x] = i + 1;
        }
        for &x in &ans {
            print!("{} ", x);
        }
        println!();
    } else {
        println!("No");
    }
}

fn solve(adj: &[Vec<usize>], deg: &mut [i32]) -> Option<Vec<usize>> {
    let mut head = deg
        .iter()
        .enumerate()
        .filter_map(|(i, &d)| if d == 0 { Some(i) } else { None })
        .collect_vec();
    let mut res = vec![];
    while let Some(h) = head.pop() {
        if !head.is_empty() {
            return None;
        }
        res.push(h);
        for &v in &adj[h] {
            deg[v] -= 1;
            if deg[v] == 0 {
                head.push(v);
            }
        }
    }
    Some(res)
}
