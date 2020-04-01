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

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    x: usize,
    y: usize,
    a: usize,
    b: usize,
    c: usize,
    p: [i64; a],
    q: [i64; b],
    r: [i64; c],
  }
  let mut p = p;
  p.sort();
  let mut q = q;
  q.sort();
  let mut i = a - x;
  let mut j = b - y;
  use std::collections::BinaryHeap;
  let mut h: BinaryHeap<_> = r.into_iter().collect();
  let mut sum_clear = 0;
  while let Some(clear) = h.pop() {
    if i == a && j == b {
      break;
    }
    if j == b || p[i] < q[j] {
      if clear > p[i] {
        i += 1;
        sum_clear += clear;
      } else {
        break;
      }
    } else {
      if clear > q[j] {
        j += 1;
        sum_clear += clear;
      } else {
        break;
      }
    }
  }
  let mut sum = 0;
  for ii in i .. a {
    sum += p[ii];
  }
  for jj in j .. b {
    sum += q[jj];
  }

  puts!("{}\n", sum + sum_clear);
}
