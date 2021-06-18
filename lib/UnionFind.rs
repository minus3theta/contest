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
