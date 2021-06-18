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

pub struct WeightedUnionFind<T> {
    par: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<T>,
}

impl<T> WeightedUnionFind<T>
where
    T: Clone + num::Zero + std::ops::AddAssign + std::ops::Sub<Output = T>,
{
    pub fn new(n: usize) -> Self {
        let mut uf = WeightedUnionFind {
            par: vec![0; n],
            rank: vec![0; n],
            diff_weight: vec![num::zero(); n],
        };
        for (i, item) in uf.par.iter_mut().enumerate() {
            *item = i;
        }
        uf
    }
    fn weight(&mut self, x: usize) -> T {
        self.find(x);
        self.diff_weight[x].clone()
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let root = self.find(self.par[x]);
            let px = self.par[x];
            let v = self.diff_weight[px].clone();
            self.diff_weight[x] += v;
            self.par[x] = root;
            self.par[x]
        }
    }
    pub fn unite(&mut self, x: usize, y: usize, mut w: T) -> bool {
        use std::cmp::Ordering;
        w += self.weight(x) - self.weight(y);
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return false;
        }
        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Less => {
                self.par[x] = y;
                self.diff_weight[x] = num::zero::<T>() - w;
            }
            Ordering::Greater => {
                self.par[y] = x;
                self.diff_weight[y] = w;
            }
            Ordering::Equal => {
                self.par[y] = x;
                self.rank[x] += 1;
                self.diff_weight[y] = w;
            }
        }
        true
    }
    pub fn same(&mut self, x: usize, y: usize) -> Option<T> {
        if self.find(x) == self.find(y) {
            Some(self.weight(y) - self.weight(x))
        } else {
            None
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        qs: [(i32, Usize1, Usize1, i64); q],
    }
    let mut uf = WeightedUnionFind::<i64>::new(n * 2);
    for &(t, x, y, v) in &qs {
        if t == 0 {
            uf.unite(x * 2, y * 2 + 1, -v);
            uf.unite(y * 2, x * 2 + 1, -v);
        } else {
            if let Some(w) = uf.same(x * 2, y * 2) {
                println!("{}", w + v);
            } else if let Some(w) = uf.same(x * 2 + 1, y * 2) {
                println!("{}", w - v);
            } else {
                println!("Ambiguous");
            }
        }
    }
}
