#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[allow(unused_macros)]
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

struct Segtree<T, F> {
  n: usize,
  dat: Vec<T>,
  op: F,
  unit: T,
}

impl<T: Clone, F: Fn(&T, &T) -> T> Segtree<T, F> {
  #[allow(dead_code)]
  fn new(n: usize, op: F, unit: T) -> Self {
    Segtree {
      n: n,
      dat: vec![unit.clone(); 2 * n.next_power_of_two() - 1],
      op: op,
      unit: unit,
    }
  }
  #[allow(dead_code)]
  fn from_vec(v: Vec<T>, op: F, unit: T) -> Self {
    use std::iter::repeat;
    let n = v.len();
    let base = n.next_power_of_two();
    let mut dat: Vec<_> = repeat(unit.clone()).take(base - 1)
      .chain(v.into_iter())
      .chain(repeat(unit.clone()).take(base - n))
      .collect();
    assert_eq!(dat.len(), 2 * base - 1);
    for i in (0 .. base - 1).rev() {
      dat[i] = op(&dat[i * 2 + 1], &dat[i * 2 + 2]);
    }
    Segtree {
      n: base,
      dat: dat,
      op: op,
      unit: unit,
    }
  }
  fn update(&mut self, mut k: usize, x: T) {
    k += self.n - 1;
    self.dat[k] = x;
    while k > 0 {
      k = (k - 1) / 2;
      self.dat[k] = (self.op)(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
    }
  }
  fn accum(&self, a: usize, b: usize) -> T {
    self.accum_inner(a, b, 0, 0, self.n)
  }
  fn accum_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
    if r <= a || b <= l {
      return self.unit.clone();
    }
    if a <= l && r <= b {
      return self.dat[k].clone();
    }
    let vl = self.accum_inner(a, b, k * 2 + 1, l, (l + r) / 2);
    let vr = self.accum_inner(a, b, k * 2 + 2, (l + r) / 2, r);
    (self.op)(&vl, &vr)
  }
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  let st = {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
  };
  let mut iter = st.split_whitespace();
  input_inner! {
    iter,
    _n: usize,
    s: chars,
    q: usize,
  }
  let s: Vec<_> = s.into_iter().map(|c| 1 << (c as i32 - 'a' as i32)).collect();
  let mut tree = Segtree::from_vec(s, |x, y| {x | y}, 0);
  for _ in 0 .. q {
    input_inner! {
      iter,
      ty: i32,
    }
    if ty == 1 {
      input_inner! {
        iter,
        i: usize1,
        c: chars,
      }
      tree.update(i, 1 << (c[0] as i32 - 'a' as i32));
    } else {
      input_inner! {
        iter,
        l: usize1,
        r: usize,
      }
      let mut pat = tree.accum(l, r);
      let mut pop = 0;
      while pat > 0 {
        if pat & 1 != 0 {
          pop += 1;
        }
        pat >>= 1;
      }
      puts!("{}\n", pop);
    }
  }
}
