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

use union_find::*;
pub mod union_find {
    #[derive(Clone, Debug)]
    pub struct UnionFind {
        par: Vec<usize>,
        rank: Vec<usize>,
    }
    impl UnionFind {
        pub fn new(n: usize) -> Self {
            let mut uf = UnionFind {
                par: vec![0; n],
                rank: vec![0; n],
            };
            for (i, item) in uf.par.iter_mut().enumerate() {
                *item = i;
            }
            uf
        }
        fn find(&mut self, x: usize) -> usize {
            if self.par[x] == x {
                x
            } else {
                let px = self.par[x];
                self.par[x] = self.find(px);
                self.par[x]
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) {
            use std::cmp::Ordering;
            let x = self.find(x);
            let y = self.find(y);
            match self.rank[x].cmp(&self.rank[y]) {
                Ordering::Less => self.par[x] = y,
                Ordering::Greater => self.par[y] = x,
                Ordering::Equal => {
                    self.par[y] = x;
                    self.rank[x] += 1;
                }
            }
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }
    let mut adj = vec![vec![]; n];
    for (a, b) in edges {
        adj[a].push(b);
    }
    let mut ans = vec![];
    let mut connect = 0;
    let mut uf = UnionFind::new(n);
    for (i, es) in adj.into_iter().enumerate().rev() {
        ans.push(connect);
        connect += 1;
        for v in es {
            if !uf.same(i, v) {
                connect -= 1;
                uf.unite(i, v);
            }
        }
    }
    for c in ans.into_iter().rev() {
        println!("{}", c);
    }
}
