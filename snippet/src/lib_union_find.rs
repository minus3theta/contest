use cargo_snippet::snippet;

#[snippet(prefix = "use union_find::*;")]
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

#[snippet(prefix = "use union_find_with_size::*;")]
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
    }
}

#[snippet(prefix = "use weighted_union_find::*;")]
pub mod weighted_union_find {
    #[derive(Clone, Debug)]
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
}
