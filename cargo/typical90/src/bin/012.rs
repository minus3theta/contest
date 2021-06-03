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

const DIRS: &[(usize, usize)] = &[(0, 1), (1, 0), (0, std::usize::MAX), (std::usize::MAX, 0)];

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut fld = vec![vec![false; w]; h];
    let mut uf = UnionFind::new(h * w);
    let id = |r: usize, c: usize| r * w + c;
    for _ in 0..q {
        input! {
            t: i32,
        }
        if t == 1 {
            input! {
                r: Usize1,
                c: Usize1,
            }
            fld[r][c] = true;
            for &(dr, dc) in DIRS {
                let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if nr < h && nc < w && fld[nr][nc] {
                    uf.unite(id(r, c), id(nr, nc));
                }
            }
        } else {
            input! {
                ra: Usize1,
                ca: Usize1,
                rb: Usize1,
                cb: Usize1,
            }
            if fld[ra][ca] && fld[rb][cb] && uf.same(id(ra, ca), id(rb, cb)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
