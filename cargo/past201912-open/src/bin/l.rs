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

fn kruskal(n: usize, edges: &mut Vec<(f64, usize, usize)>) -> f64 {
    edges.sort_by_key(|&(cost, _, _)| ordered_float::NotNan::new(cost).unwrap());
    let mut uf = UnionFind::new(n);
    let mut ans = 0.0;
    for &mut (cost, u, v) in edges {
        if !uf.same(u, v) {
            uf.unite(u, v);
            ans += cost;
        }
    }
    ans
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        tower: [(f64, f64, i32); n+m],
    }
    let mut ans = 1e100_f64;
    for p in 0..(1 << m) {
        let mut edges = vec![];
        let mut count = 0;
        for i in 0..n + m {
            if i >= n && (p >> (i - n) & 1) == 1 {
                continue;
            }
            count += 1;
            for j in 0..n + m {
                if j >= n && (p >> (j - n) & 1) == 1 {
                    continue;
                }
                let (x1, y1, c1) = tower[i];
                let (x2, y2, c2) = tower[j];
                let cost = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
                let cost = if c1 == c2 { cost } else { cost * 10.0 };
                edges.push((cost, i, j));
            }
        }
        ans = ans.min(kruskal(count, &mut edges));
    }

    println!("{:.15}", ans);
}
