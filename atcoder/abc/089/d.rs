#[allow(unused_imports)]
use std::cmp;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
    let mut iter = $s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
    #[allow(unused_mut)]
    let mut s = {
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

fn cost(ij: (i32, i32), xy: (i32, i32)) -> i32 {
  (xy.0 - ij.0).abs() + (xy.1 - ij.1).abs()
}

fn main() {
  input! {
    h: usize,
    w: usize,
    d: usize,
    field: [[usize1; w]; h],
    q: usize,
    query: [(usize1, usize1); q],
  }
  let mut rev = vec![(0, 0); h * w];
  for i in 0 .. h {
    for j in 0 .. w {
      rev[field[i][j]] = (i as i32, j as i32);
    }
  }
  let mut total = vec![0; h * w];
  for k in d .. h * w {
    total[k] = total[k - d] + cost(rev[k - d], rev[k]);
  }
  for &(l, r) in &query {
    println!("{}", total[r] - total[l]);
  }
}
