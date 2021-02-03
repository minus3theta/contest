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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Clone)]
struct Vertex {
    parent: Option<usize>,
    children: Vec<usize>,
    score: i64,
}

fn dfs(vs: &mut [Vertex], adj: &[Vec<usize>], v: usize, parent: Option<usize>) {
    vs[v].parent = parent;
    for &u in &adj[v] {
        if parent == Some(u) {
            continue;
        }
        vs[v].children.push(u);
        dfs(vs, adj, u, Some(v));
    }
}

fn resolve(vs: &[Vertex], score: &mut [i64], v: usize, offset: i64) {
    score[v] = offset + vs[v].score;
    for &u in &vs[v].children {
        resolve(vs, score, u, offset + vs[v].score);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        es: [(Usize1, Usize1); n - 1],
        q: usize,
        qs: [(i32, Usize1, i64); q],
    }
    let mut adj = vec![vec![]; n];
    for &(a, b) in &es {
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut vs = vec![Vertex::default(); n];
    dfs(&mut vs, &adj, 0, None);
    let mut offset = 0;
    for &(t, e, x) in &qs {
        let (src, ban) = if t == 1 { es[e] } else { (es[e].1, es[e].0) };
        if vs[src].parent == Some(ban) {
            vs[src].score += x;
        } else {
            offset += x;
            vs[ban].score -= x;
        }
    }
    let mut score = vec![0; n];
    resolve(&vs, &mut score, 0, offset);
    for &s in &score {
        println!("{}", s);
    }
}
