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

struct UnionFind {
  par: Vec<usize>,
  size: Vec<usize>,
}
use std::cmp::Ordering;

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
    match self.size[x].cmp(&self.size[y]) {
      Ordering::Less => {
        self.par[x] = y;
        self.size[y] += self.size[x];
      },
      _ => {
        self.par[y] = x;
        self.size[x] += self.size[y];
      }
    }
  }
  fn same(&mut self, x: usize, y: usize) -> bool {
    self.find(x) == self.find(y)
  }
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
    k: usize,
    fr: [(usize1, usize1); m],
    bl: [(usize1, usize1); k],
  }
  let mut uf = UnionFind::new(n);
  for &(a, b) in &fr {
    uf.unite(a, b);
  }
  let mut fr_a = vec![vec![]; n];
  let mut bl_a = vec![vec![]; n];
  for &(a, b) in &fr {
    fr_a[a].push(b);
    fr_a[b].push(a);
  }
  for &(c, d) in &bl {
    bl_a[c].push(d);
    bl_a[d].push(c);
  }
  for i in 0 .. n {
    let root = uf.find(i);
    let mut s = uf.size[root] - fr_a[i].len() - 1;
    for &b in &bl_a[i] {
      if uf.same(i, b) {
        s -= 1;
      }
    }
    puts!("{} ", s);
  }

  puts!("\n");
}
