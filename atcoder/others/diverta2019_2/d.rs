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

fn gain(way: &Vec<(usize, usize)>, init: usize) -> usize {
  let mut g = vec![0usize; init + 1];
  for i in 0 .. init + 1 {
    g[i] = i;
  }
  for i in 1 .. init + 1 {
    for &(before, after) in way {
      if let Some(j) = i.checked_sub(before) {
        g[i] = cmp::max(g[i], g[j] + after);
      }
    }
  }
  g[init]
}

use cmp::Ordering::*;

fn main() {
  input! {
    n: usize,
    v: [[usize; 3]; 2],
  }
  let mut ab = vec![];
  let mut ba = vec![];
  for i in 0 .. 3 {
    match v[0][i].cmp(&v[1][i]) {
      Less => ab.push((v[0][i], v[1][i])),
      Greater => ba.push((v[1][i], v[0][i])),
      _ => ()
    }
  }
  let temp = gain(&ab, n);
  let ans = gain(&ba, temp);

  println!("{}", ans);
}
