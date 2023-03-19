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

use union_find_with_size::*;
pub mod union_find_with_size {
    #[derive(Clone, Debug)]
    pub struct UnionFindWithSize {
        pub par: Vec<usize>,
        pub size: Vec<usize>,
        pub edges: Vec<usize>,
    }
    impl UnionFindWithSize {
        pub fn new(n: usize) -> Self {
            let mut uf = UnionFindWithSize {
                par: vec![0; n],
                size: vec![1; n],
                edges: vec![0; n],
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
            let x = self.find(x);
            let y = self.find(y);
            if x == y {
                self.edges[x] += 1;
                return;
            }
            if self.size[x] < self.size[y] {
                self.par[x] = y;
                self.size[y] += self.size[x];
                self.edges[y] += self.edges[x] + 1;
            } else {
                self.par[y] = x;
                self.size[x] += self.size[y];
                self.edges[x] += self.edges[y] + 1;
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
        es: [(Usize1, Usize1); m],
    }
    let mut uf = UnionFindWithSize::new(n);
    for &(u, v) in &es {
        uf.unite(u, v);
    }
    let ans = (0..n).all(|v| uf.par[v] != v || uf.size[v] == uf.edges[v]);

    println!("{}", if ans { "Yes" } else { "No" });
}
