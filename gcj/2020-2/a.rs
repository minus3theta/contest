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

// sum of i in [0, s)
fn tri(s: i64) -> i64 {
  s * (s - 1) / 2
}

// max n s.t. tri(n) <= x
fn take(x: i64) -> (i64, i64) {
  let mut lb = 0;
  let mut ub = 1 << 30;
  while lb + 1 != ub {
    let m = (lb + ub) / 2;
    if tri(m) <= x {
      lb = m;
    } else {
      ub = m;
    }
  }
  (lb, tri(lb))
}

fn tri_skip(n: i64, x: i64) -> i64 {
  n * (x + n - 1)
}

fn take_alt(n: i64, l: i64, r: i64) -> (i64, i64, i64) {
  let mut lb = 0;
  let mut ub = 1 << 30;
  while lb + 1 != ub {
    let m = (lb + ub) / 2;
    if tri_skip(m, n) <= l && tri_skip(m, n+1) <= r {
      lb = m;
    } else {
      ub = m;
    }
  }
  (n+2*lb, l - tri_skip(lb, n), r - tri_skip(lb, n+1))
}

fn solve(mut l: i64, mut r: i64) -> (i64, i64, i64) {
  let n = if l >= r {
    let diff = l - r;
    let (n, t) = take(diff);
    l -= t;
    n
  } else {
    let diff = r - l;
    let (n, t) = take(diff-1);
    r -= t;
    if n > r {
      return (n-1, l, r);
    }
    r -= n;
    n+1
  };
  assert!(l >= r);
  assert!(l - n < r);
  let (mut n, mut l, r) = take_alt(n, l, r);
  if l >= n {
    l -= n;
    n += 1;
  }
  (n-1, l, r)
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    t: usize,
    lr: [(i64, i64); t],
  }
  for (c, &(l, r)) in lr.iter().enumerate() {
    let (n, l, r) = solve(l, r);
    puts!("Case #{}: {} {} {}\n", c+1, n, l, r);
  }
}
