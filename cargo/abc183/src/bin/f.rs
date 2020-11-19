use cmp::Ordering;
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

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    class: Vec<BTreeMap<i32, i32>>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut uf = UnionFind {
            par: vec![0; n],
            rank: vec![0; n],
            class: vec![BTreeMap::new(); n],
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
    fn unite(&mut self, x: usize, y: usize) {
        if self.same(x, y) {
            return;
        }
        let x = self.find(x);
        let y = self.find(y);
        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Less => {
                self.merge(y, x);
                self.par[x] = y;
            }
            Ordering::Greater => {
                self.merge(x, y);
                self.par[y] = x;
            }
            Ordering::Equal => {
                self.merge(x, y);
                self.par[y] = x;
                self.rank[x] += 1;
            }
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn merge(&mut self, x: usize, y: usize) {
        let mut set = BTreeMap::new();
        std::mem::swap(&mut set, &mut self.class[y]);
        for (&i, &c) in &set {
            *self.class[x].entry(i).or_insert(0) += c;
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        cl: [i32; n],
    }
    let mut uf = UnionFind::new(n);
    for (i, &c) in cl.iter().enumerate() {
        uf.class[i].insert(c, 1);
    }
    for _ in 0..q {
        input! {
            ty: i32,
        }
        match ty {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                uf.unite(a, b);
            }
            _ => {
                input! {
                    x: Usize1,
                    y: i32,
                }
                let root = uf.find(x);
                println!("{}", uf.class[root].get(&y).unwrap_or(&0));
            }
        }
    }
}
