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

fn to_index(r: usize, c: usize) -> usize {
    r * 4 + c
}

fn is_filled(pat: i32, r: usize, c: usize) -> bool {
    is_filled_index(pat, to_index(r, c))
}

fn is_filled_index(pat: i32, index: usize) -> bool {
    (pat >> index) & 1 == 1
}

#[fastout]
fn main() {
    input! {
        a: [[u8; 4]; 4],
    }
    let villeges = a
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &cell)| if cell == 1 { Some((r, c)) } else { None })
        })
        .collect_vec();
    let includes = |pat: i32| villeges.iter().all(|&(r, c)| is_filled(pat, r, c));
    let connected = |pat: i32| {
        let mut uf = UnionFind::new(16 + 1);
        for r in 0..4 {
            for c in 0..4 - 1 {
                if is_filled(pat, r, c) == is_filled(pat, r, c + 1) {
                    uf.unite(to_index(r, c), to_index(r, c + 1));
                }
            }
        }
        for r in 0..4 - 1 {
            for c in 0..4 {
                if is_filled(pat, r, c) == is_filled(pat, r + 1, c) {
                    uf.unite(to_index(r, c), to_index(r + 1, c));
                }
            }
        }
        for r in 0..4 {
            for c in 0..4 {
                if !is_filled(pat, r, c) && (r == 0 || r == 3 || c == 0 || c == 3) {
                    uf.unite(to_index(r, c), 16);
                }
            }
        }
        (0..=16)
            .map(|i| Some(uf.find(i)))
            .collect::<HashSet<_>>()
            .len()
            == 2
    };
    let ans = (0..1 << 16)
        .filter(|&pat| includes(pat) && connected(pat))
        .count();

    println!("{}", ans);
}
