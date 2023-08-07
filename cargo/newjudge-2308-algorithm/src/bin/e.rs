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
        k: usize,
        xy: [(Usize1, Usize1); k],
        q: usize,
        query: [(Usize1, Usize1); q],
    }
    let mut uf = UnionFind::new(n);
    for &(u, v) in &es {
        uf.unite(u, v);
    }
    let mut to_pair = |x: usize, y: usize| -> (usize, usize) {
        let mut x = uf.find(x);
        let mut y = uf.find(y);
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        (x, y)
    };
    let set = xy
        .iter()
        .map(|&(x, y)| to_pair(x, y))
        .collect::<BTreeSet<_>>();
    for &(p, q) in &query {
        println!(
            "{}",
            if set.contains(&to_pair(p, q)) {
                "No"
            } else {
                "Yes"
            }
        );
    }
}

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
        pub fn find(&mut self, x: usize) -> usize {
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
