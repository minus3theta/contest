#[derive(Clone, Debug)]
struct UnionFind {
  par: Vec<usize>,
  size: Vec<usize>,
}

impl UnionFind {
  fn new(n: usize) -> Self {
    let mut uf = UnionFind {par: vec![0; n], size: vec![1; n]};
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
  fn same(&mut self, x: usize, y: usize) -> bool {
    self.find(x) == self.find(y)
  }
}
