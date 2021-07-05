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
        q: usize,
        qs: [(Usize1, i64); q],
    }
    let mut adj = vec![vec![]; n];
    for &(a, b) in &es {
        adj[a].push(b);
        adj[b].push(a);
    }
    let threshold = ((2 * m) as f64).sqrt() as usize;
    let mut large_adj = vec![vec![]; n];
    for i in 0..n {
        for &v in &adj[i] {
            if adj[v].len() >= threshold {
                large_adj[i].push(v);
            }
        }
    }
    let mut color = vec![1; n];
    let mut last_update = vec![None::<usize>; n];
    for (t, &(x, y)) in qs.iter().enumerate() {
        let col = if adj[x].len() < threshold {
            let upd = adj[x]
                .iter()
                .chain(&Some(x))
                .filter_map(|&v| last_update[v])
                .max();
            if let Some(upd) = upd {
                qs[upd].1
            } else {
                1
            }
        } else {
            color[x]
        };
        println!("{}", col);

        color[x] = y;
        for &v in &large_adj[x] {
            color[v] = y;
        }
        last_update[x] = Some(t);
    }
}
