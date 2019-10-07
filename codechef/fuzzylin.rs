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

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

fn gcd(x: i64, y: i64) -> i64 {
  if y == 0 {
    x
  } else {
    gcd(y, x % y)
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
    a: [i64; n],
    q: usize,
    ks: [usize; q],
  }
  use std::collections::HashMap;
  let map_all = {
    let mut map_all = HashMap::new();
    let mut prev = HashMap::new();
    for i in 0 .. n {
      let mut map_current = HashMap::new();
      for (&g, &count) in &prev {
        *map_current.entry(gcd(g, a[i])).or_insert(0) += count;
      }
      *map_current.entry(a[i]).or_insert(0) += 1i64;
      prev = map_current;
      for (&g, &count) in &prev {
        *map_all.entry(g).or_insert(0) += count;
      }
    }
    map_all
  };
  let max_k = 1_000_001;
  let mut gs = vec![0; max_k];
  for (&g, &count) in &map_all {
    let g = g as usize;
    let mut j = 1;
    while g * j < max_k {
      gs[g * j] += count;
      j += 1;
    }
  }
  for &k in &ks {
    puts!("{}\n", gs[k]);
  }
}
