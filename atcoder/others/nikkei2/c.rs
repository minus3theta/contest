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

use std::cmp::Ordering;

struct UnionFind {
  par: Vec<usize>,
  rank: Vec<usize>,
}

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
  #[allow(dead_code)]
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
    aa: [i64; n],
    bb: [i64; n],
  }
  let mut ba: Vec<_> = bb.iter().cloned().zip(aa.iter().cloned()).collect();
  ba.sort();
  let mut aa = aa;
  aa.sort();
  let mut bb = bb;
  bb.sort();
  let mut ok = true;
  let mut multi = false;
  for i in 0 .. n {
    if aa[i] > bb[i] {
      ok = false;
      break;
    }
    if i > 0 && aa[i] <= bb[i-1] {
      multi = true;
    }
  }
  if ! ok {
    puts!("No\n");
    return;
  }
  if multi {
    puts!("Yes\n");
    return;
  }
  let mut perm: Vec<_> = (0 .. n).collect();
  perm.sort_by_key(|&i| ba[i].1);
  let mut uf = UnionFind::new(n);
  for (i, &j) in perm.iter().enumerate() {
    uf.unite(i, j);
  }
  let groups = (0 .. n).filter(|&i| i == uf.find(i)).count();
  if groups > 1 {
    puts!("Yes\n");
  } else {
    puts!("No\n");
  }
}
