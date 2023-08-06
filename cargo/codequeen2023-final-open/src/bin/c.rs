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
        s: Usize1,
        t: Usize1,
        es: [(Usize1, Usize1); n-1],
    }
    let mut adj = vec![vec![]; n];
    for &(u, v) in &es {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut euler = Vec::new();
    let mut vertex_index = vec![0; n];
    dfs(s, None, 0, &adj, &mut euler, &mut vertex_index);
    let lca = euler
        .iter()
        .map(|&Euler { vertex, depth, .. }| (vertex, depth))
        .collect();
    let lca = Segtree::from_vec(
        lca,
        |&(v1, d1), &(v2, d2)| if d1 < d2 { (v1, d1) } else { (v2, d2) },
        (0, std::usize::MAX),
    );

    for j in 0..n {
        let j = vertex_index[j];
        let t = vertex_index[t];
        let left = j.min(t);
        let right = j.max(t);
        let j = lca.accum(j, j + 1);
        let lca = lca.accum(left, right + 1);
        println!("{}", j.1 - lca.1 + 1);
    }
}

#[derive(Debug)]
struct Euler {
    vertex: usize,
    depth: usize,
}

impl Euler {
    fn new(vertex: usize, depth: usize) -> Self {
        Self { vertex, depth }
    }
}

fn dfs(
    v: usize,
    prev: Option<usize>,
    depth: usize,
    adj: &[Vec<usize>],
    euler: &mut Vec<Euler>,
    vertex_index: &mut [usize],
) {
    vertex_index[v] = euler.len();
    euler.push(Euler::new(v, depth));
    for &u in &adj[v] {
        if Some(u) == prev {
            continue;
        }
        euler.push(Euler::new(v, depth));
        dfs(u, Some(v), depth + 1, adj, euler, vertex_index);
        euler.push(Euler::new(v, depth));
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
