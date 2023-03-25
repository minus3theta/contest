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
        es: [(Usize1, Usize1, i64); n-1],
        q: usize,
    }
    let mut adj = vec![vec![]; n];
    for (i, (u, v, w)) in es.into_iter().enumerate() {
        adj[u].push((i, v, w));
        adj[v].push((i, u, w));
    }
    let mut euler = Vec::new();
    let mut vertex_index = vec![0; n];
    let mut edge_index = vec![(0, 0); n - 1];
    dfs(
        0,
        None,
        0,
        &adj,
        &mut euler,
        &mut vertex_index,
        &mut edge_index,
    );
    let edge = euler.iter().map(|&Euler { weight, .. }| weight).collect();
    let mut edge = Bit::from_vec(edge, |x, y| *x += *y, 0);
    let lca = euler
        .iter()
        .map(|&Euler { vertex, depth, .. }| (vertex, depth))
        .collect();
    let lca = Segtree::from_vec(
        lca,
        |&(v1, d1), &(v2, d2)| if d1 < d2 { (v1, d1) } else { (v2, d2) },
        (0, std::usize::MAX),
    );

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                i: Usize1,
                w: i64,
            }
            let (f, b) = edge_index[i];
            let current = edge.accum(f + 1) - edge.accum(f);
            let diff = w - current;
            edge.operate(f, &diff);
            edge.operate(b, &-diff);
        } else {
            input! {
                u: Usize1,
                v: Usize1,
            }
            let u = vertex_index[u];
            let v = vertex_index[v];
            let left = u.min(v);
            let right = u.max(v) + 1;
            let w = if u == v {
                u
            } else {
                let w = lca.accum(left, right).0;
                vertex_index[w]
            };
            let dist = edge.accum(u) + edge.accum(v) - 2 * edge.accum(w);
            println!("{}", dist);
        }
    }
}

#[derive(Debug)]
struct Euler {
    vertex: usize,
    depth: usize,
    weight: i64,
}

impl Euler {
    fn new(vertex: usize, depth: usize, weight: i64) -> Self {
        Self {
            vertex,
            depth,
            weight,
        }
    }
}

fn dfs(
    v: usize,
    prev: Option<usize>,
    depth: usize,
    adj: &[Vec<(usize, usize, i64)>],
    euler: &mut Vec<Euler>,
    vertex_index: &mut [usize],
    edge_index: &mut [(usize, usize)],
) {
    vertex_index[v] = euler.len();
    for &(i, u, w) in &adj[v] {
        if Some(u) == prev {
            continue;
        }
        edge_index[i].0 = euler.len();
        euler.push(Euler::new(v, depth, w));
        dfs(u, Some(v), depth + 1, adj, euler, vertex_index, edge_index);
        edge_index[i].1 = euler.len();
        euler.push(Euler::new(v, depth, -w));
    }
}

use segtree::*;
pub mod segtree {
    pub struct Segtree<T, F> {
        n: usize,
        dat: Vec<T>,
        op: F,
        unit: T,
    }
    impl<T: Clone, F: Fn(&T, &T) -> T> Segtree<T, F> {
        pub fn new(n: usize, op: F, unit: T) -> Self {
            Segtree {
                n: n.next_power_of_two(),
                dat: vec![unit.clone(); 2 * n.next_power_of_two() - 1],
                op,
                unit,
            }
        }
        pub fn from_vec(v: Vec<T>, op: F, unit: T) -> Self {
            use std::iter::repeat;
            let n = v.len();
            let base = n.next_power_of_two();
            let mut dat: Vec<_> = repeat(unit.clone())
                .take(base - 1)
                .chain(v.into_iter())
                .chain(repeat(unit.clone()).take(base - n))
                .collect();
            assert_eq!(dat.len(), 2 * base - 1);
            for i in (0..base - 1).rev() {
                dat[i] = op(&dat[i * 2 + 1], &dat[i * 2 + 2]);
            }
            Segtree {
                n: base,
                dat,
                op,
                unit,
            }
        }
        pub fn update(&mut self, mut k: usize, x: T) {
            k += self.n - 1;
            self.dat[k] = x;
            while k > 0 {
                k = (k - 1) / 2;
                self.dat[k] = (self.op)(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
            }
        }
        pub fn accum(&self, a: usize, b: usize) -> T {
            self.accum_inner(a, b, 0, 0, self.n)
        }
        fn accum_inner(&self, a: usize, b: usize, k: usize, left: usize, right: usize) -> T {
            if right <= a || b <= left {
                return self.unit.clone();
            }
            if a <= left && right <= b {
                return self.dat[k].clone();
            }
            let vl = self.accum_inner(a, b, k * 2 + 1, left, (left + right) / 2);
            let vr = self.accum_inner(a, b, k * 2 + 2, (left + right) / 2, right);
            (self.op)(&vl, &vr)
        }
    }
}

use bit::*;
pub mod bit {
    /// 0-indexed
    /// example: Bit::new(n, |x, y| *x += *y, 0)
    pub struct Bit<T, F> {
        n: usize,
        dat: Vec<T>,
        op: F,
        unit: T,
    }
    impl<T: Clone, F: Fn(&mut T, &T)> Bit<T, F> {
        pub fn new(n: usize, op: F, unit: T) -> Self {
            Bit {
                n,
                dat: vec![unit.clone(); n + 1],
                op,
                unit,
            }
        }
        pub fn from_vec(mut v: Vec<T>, op: F, unit: T) -> Self {
            let n = v.len();
            let mut dat = vec![unit.clone()];
            dat.append(&mut v);
            for i in 1..n {
                let b = {
                    let i = i as i32;
                    (i & -i) as usize
                };
                let x = dat[i].clone();
                if i + b <= n {
                    op(&mut dat[i + b], &x);
                }
            }
            Bit { n, dat, op, unit }
        }
        /// operates `a` to `k`-th element.
        pub fn operate(&mut self, k: usize, a: &T) {
            let mut k = k + 1;
            while k <= self.n {
                (self.op)(&mut self.dat[k], a);
                let l = k as i32;
                k += (l & -l) as usize;
            }
        }
        /// accumulates [0..k)
        pub fn accum(&self, k: usize) -> T {
            let mut k = k;
            let mut sum = self.unit.clone();
            while k > 0 {
                (self.op)(&mut sum, &self.dat[k]);
                let l = k as i32;
                k -= (l & -l) as usize;
            }
            sum
        }
    }
}
