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
}

impl UnionFind {
    fn new(n: usize) -> Self {
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
    fn unite(&mut self, x: usize, y: usize) {
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
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

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
    fn accum_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.unit.clone();
        }
        if a <= l && r <= b {
            return self.dat[k].clone();
        }
        let vl = self.accum_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.accum_inner(a, b, k * 2 + 2, (l + r) / 2, r);
        (self.op)(&vl, &vr)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        qs: [(i32, Usize1, Usize1, i64); q],
    }
    let mut uf = UnionFind::new(n);
    let mut seg = Segtree::new(n - 1, |x, y| x + y, 0i64);
    for &(t, x, y, v) in &qs {
        if t == 0 {
            uf.unite(x, y);
            seg.update(x, if x % 2 == 0 { -v } else { v });
        } else {
            if uf.same(x, y) {
                let v = if x % 2 == 0 { v } else { -v };
                let z = if x < y {
                    v + seg.accum(x, y)
                } else {
                    v - seg.accum(y, x)
                };
                let z = if y % 2 == 0 { z } else { -z };
                println!("{}", z);
            } else {
                println!("Ambiguous");
            }
        }
    }
}
