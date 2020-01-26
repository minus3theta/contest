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

// 1-indexed
// example: Bit::new(n, |x, y| *x += *y, 0)
struct Bit<T, F> {
  n: usize,
  dat: Vec<T>,
  op: F,
  unit: T,
}

impl<T: Clone, F: Fn(&mut T, &T)> Bit<T, F> {
  #[allow(dead_code)]
  fn new(n: usize, op: F, unit: T) -> Self {
      Bit {
          n: n,
          dat: vec![unit.clone(); n + 1],
          op: op,
          unit: unit,
      }
  }
  #[allow(dead_code)]
  fn from_vec(mut v: Vec<T>, op: F, unit: T) -> Self {
      let n = v.len();
      let mut dat = vec![unit.clone()];
      dat.append(&mut v);
      for i in 1..n {
          let j = i as i32;
          let b = (j & -j) as usize;
          let x = dat[i].clone();
          op(&mut dat[i + b], &x);
      }
      Bit {
          n: n,
          dat: dat,
          op: op,
          unit: unit,
      }
  }
  fn operate(&mut self, k: usize, a: &T) {
      let mut k = k;
      while k <= self.n {
          (self.op)(&mut self.dat[k], &a);
          let l = k as i32;
          k += (l & -l) as usize;
      }
  }
  fn accum(&self, k: usize) -> T {
      let mut k = k;
      let mut sum = self.unit.clone();
      while k > 0 {
          (self.op)(&mut sum, &self.dat[k]);
          let l = k as i32;
          k -= (l & -l) as usize;
      }
      sum
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
    d: i64,
    a: i64,
    mon: [(i64, i64); n],
  }
  let mut mon = mon;
  mon.sort();
  for x in mon.iter_mut() {
    x.1 = (x.1 - 1) / a + 1;
  }
  let mut bit = Bit::new(n, |x, y| *x += *y, 0);
  let mut prev = 0;
  for (i, &(_, h)) in mon.iter().enumerate() {
    bit.operate(i+1, &(h - prev));
    prev = h;
  }
  let mut r = 0;
  let mut ans = 0;
  for i in 0 .. n {
    while r < n && mon[i].0 + 2 * d >= mon[r].0 {
      r += 1;
    }
    let h = bit.accum(i+1);
    if h > 0 {
      ans += h;
      bit.operate(i+1, &-h);
      if r < n {
        bit.operate(r+1, &h);
      }
    }
  }

  puts!("{}\n", ans);
}
