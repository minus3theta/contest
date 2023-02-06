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

fn solve(n: usize, es: &[(usize, usize)]) -> bool {
    if es.len() != n - 1 {
        return false;
    }
    let mut deg = vec![0; n];
    for &(a, b) in es {
        deg[a] += 1;
        deg[b] += 1;
    }
    let count_one = deg.iter().filter(|&&c| c == 1).count();
    let count_two = deg.iter().filter(|&&c| c == 2).count();
    if count_one != 2 || count_two != n - 2 {
        return false;
    }
    let mut uf = UnionFind::new(n);
    for &(a, b) in es {
        uf.unite(a, b);
    }
    (1..n).all(|i| uf.same(0, i))
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }

    println!("{}", if solve(n, &es) { "Yes" } else { "No" });
}
