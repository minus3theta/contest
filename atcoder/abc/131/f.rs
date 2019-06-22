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

const K: usize = 100_000;

fn dfs(i: usize, edge: &Vec<Vec<usize>>, used: &mut Vec<bool>, count: &mut [i64; 2]) -> i64 {
  used[i] = true;
  count[i / K] += 1;
  for &j in &edge[i] {
    if ! used[j] {
      dfs(j, edge, used, count);
    }
  }
  count[0] * count[1]
}

fn main() {
  input! {
    n: usize,
    p: [(usize1, usize1); n],
  }
  let mut edge = vec![vec![]; 2 * K];
  for &(x, y) in &p {
    let y = y + K;
    edge[x].push(y);
    edge[y].push(x);
  }
  let mut used = vec![false; 2 * K];
  let mut ans = 0;
  for i in 0 .. 2 * K {
    ans += dfs(i, &edge, &mut used, &mut [0, 0]);
  }

  println!("{}", ans - n as i64);
}
