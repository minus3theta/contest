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
        par: Vec<usize>,
        size: Vec<usize>,
    }
    impl UnionFindWithSize {
        pub fn new(n: usize) -> Self {
            let mut uf = UnionFindWithSize {
                par: vec![0; n],
                size: vec![1; n],
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
                return;
            }
            if self.size[x] < self.size[y] {
                self.par[x] = y;
                self.size[y] += self.size[x];
            } else {
                self.par[y] = x;
                self.size[x] += self.size[y];
            }
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.find(x);
            self.size[root]
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut es: [(Usize1, Usize1, usize); n-1],
    }
    es.sort_by_key(|&(_, _, w)| w);
    let mut uf = UnionFindWithSize::new(n);
    let mut ans = 0;
    for (u, v, w) in es {
        let su = uf.size(u);
        let sv = uf.size(v);
        ans += su * sv * w;
        uf.unite(u, v);
    }

    println!("{}", ans);
}
