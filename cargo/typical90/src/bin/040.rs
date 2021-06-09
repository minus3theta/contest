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

use std::cell::Cell;

#[derive(Debug, Clone)]
struct Edge {
    pub to: usize,
    pub cap: Cell<i64>,
    pub rev: usize,
}

fn add_edge(from: usize, to: usize, cap: i64, es: &mut Vec<Vec<Edge>>) {
    let from_len = es[from].len();
    let to_len = es[to].len();
    es[from].push(Edge {
        to,
        cap: Cell::new(cap),
        rev: to_len,
    });
    es[to].push(Edge {
        to: from,
        cap: Cell::new(0),
        rev: from_len,
    });
}

fn dfs(v: usize, t: usize, f: i64, es: &Vec<Vec<Edge>>, used: &mut Vec<bool>) -> i64 {
    if v == t {
        return f;
    }
    used[v] = true;
    for e in &es[v] {
        let cap = e.cap.get();
        if !used[e.to] && cap > 0 {
            let d = dfs(e.to, t, f.min(cap), es, used);
            if d > 0 {
                e.cap.set(cap - d);
                let r_cap = es[e.to][e.rev].cap.get();
                es[e.to][e.rev].cap.set(r_cap + d);
                return d;
            }
        }
    }
    0
}

fn max_flow(s: usize, t: usize, es: &Vec<Vec<Edge>>) -> i64 {
    let mut flow = 0;
    loop {
        let mut used = vec![false; es.len()];
        let f = dfs(s, t, 1 << 60, es, &mut used);
        if f == 0 {
            break flow;
        }
        flow += f;
    }
}

const INF: i64 = 1 << 50;
const GAIN: i64 = 1 << 30;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: i64,
        a: [i64; n],
        key: [[Usize1]; n],
    }
    let source = n;
    let sink = source + 1;
    let mut adj = vec![vec![]; n + 2];
    for (i, &x) in a.iter().enumerate() {
        add_edge(source, i, GAIN + w - x, &mut adj);
        add_edge(i, sink, GAIN, &mut adj);
    }
    for (i, cs) in key.iter().enumerate() {
        for &c in cs {
            add_edge(i, c, INF, &mut adj);
        }
    }
    let cost = max_flow(source, sink, &adj);

    println!("{}", GAIN * n as i64 - cost);
}
