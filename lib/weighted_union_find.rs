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
