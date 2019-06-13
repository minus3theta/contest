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

fn solve(n: usize, x: i64, size: &Vec<i64>, all: &Vec<i64>) -> i64 {
  if n == 0 {
    x
  } else if x <= size[n-1] + 1 {
    solve(n - 1, cmp::max(x - 1, 0), size, all)
  } else {
    all[n-1] + 1 + solve(n - 1, cmp::min(x - size[n-1] - 2, size[n-1]), size, all)
  }
}

fn main() {
  input! {
    n: usize,
    x: i64,
  }
  let mut size = vec![0i64; n+1];
  size[0] = 1;
  let mut all = vec![0i64; n+1];
  all[0] = 1;
  for i in 0 .. n {
    size[i+1] = 2 * size[i] + 3;
    all[i+1] = 2 * all[i] + 1;
  }

  println!("{}", solve(n, x, &size, &all));
}
