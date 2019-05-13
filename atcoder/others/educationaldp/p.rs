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

const MOD: i64 = 1e9 as i64 + 7;

fn solve(vs: &Vec<Vec<usize>>, x: usize, parent: Option<usize>) -> (i64, i64) {
  let mut black = 1;
  let mut white = 1;
  for &child in vs[x].iter() {
    if let Some(parent) = parent {
      if child == parent {
        continue;
      }
    }
    let (b, w) = solve(&vs, child, Some(x));
    black = black * w % MOD;
    white = white * (b + w) % MOD;
  }
  (black, white)
}

fn main() {
  input!{
    n: usize,
    es: [(usize1, usize1); n-1]
  }
  let mut vs = vec![vec![]; n];
  for &(x, y) in es.iter() {
    vs[x].push(y);
    vs[y].push(x);
  }
  let (black, white) = solve(&vs, 0, None);
  println!("{}", (black + white) % MOD);
}
