#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
    let mut iter = $s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
    let s = {
      use std::io::Read;
      let mut s = String::new();
      std::io::stdin().read_to_string(&mut s).unwrap();
      s
    };
    let mut iter = s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
}

macro_rules! input_inner {
  ($iter:expr) => {};
  ($iter:expr, ) => {};

  ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
    let $var = read_value!($iter, $t);
    input_inner!{$iter $($r)*}
  };
}

macro_rules! read_value {
  ($iter:expr, ( $($t:tt),* )) => {
    ( $(read_value!($iter, $t)),* )
  };

  ($iter:expr, [ $t:tt ; $len:expr ]) => {
    (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  };

  ($iter:expr, chars) => {
    read_value!($iter, String).chars().collect::<Vec<char>>()
  };

  ($iter:expr, usize1) => {
    read_value!($iter, usize) - 1
  };

  ($iter:expr, [ $t:tt ]) => {{
    let len = read_value!($iter, usize);
    (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  }};

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

struct UnionFind {
  par: Vec<usize>,
  rank: Vec<usize>,
}

use cmp::Ordering;

impl UnionFind {
  fn new(n: usize) -> Self {
    let mut uf = UnionFind {par: vec![0; n], rank: vec![0; n]};
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

fn mst(tow: &Vec<(f64, f64, i32)>) -> f64 {
  let n = tow.len();
  let mut edges = vec![];
  for i in 0 .. n {
    for j in 0 .. i {
      let mut d = ((tow[i].0 - tow[j].0).powi(2) + (tow[i].1 - tow[j].1).powi(2)).sqrt();
      if tow[i].2 != tow[j].2 {
        d *= 10.0;
      }
      edges.push((i, j, d));
    }
  }
  edges.sort_by_key(|&x| Total(x.2));
  let mut uf = UnionFind::new(n);
  let mut cost = 0.0;
  for &(i, j, d) in &edges {
    if uf.same(i, j) {
      continue;
    }
    uf.unite(i, j);
    cost += d;
  }
  cost
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    m: usize,
    large: [(f64, f64, i32); n],
    small: [(f64, f64, i32); m],
  }
  let mut ans = 1e100;
  for used in 0 .. (1 << m) {
    let mut tower = large.clone();
    for (i, t) in small.iter().enumerate() {
      if (used >> i) & 1 != 0 {
        tower.push(t.clone());
      }
    }
    let cost = mst(&tower);
    if cost < ans {
      ans = cost;
    }
  }

  puts!("{:.15}\n", ans);
}
