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

fn solve(x: i64, y: i64, _k: i64, comb: i64) -> i64 {
  let mut ans = 0;
  for d in 1 .. x {
    ans = (ans + d * (x - d) % MOD * y * y) % MOD;
  }
  ans * comb % MOD
}

fn factor(x: i64) -> i64 {
  (1 .. x + 1).fold(1, |f, a| f * a % MOD)
}

fn pow(x: i64, e: i64) -> i64 {
  if e == 1 {
    x
  } else if e % 2 == 0 {
    let y = pow(x, e / 2);
    y * y % MOD
  } else {
    pow(x, e - 1) * x % MOD
  }
}

fn main() {
  input! {
    n: i64,
    m: i64,
    k: i64,
  }
  let div = factor(k - 2) * factor(n * m - k) % MOD;
  let comb = factor(n * m - 2) * pow(div, MOD - 2) % MOD;
  let ans = (solve(n, m, k, comb) + solve(m, n, k, comb)) % MOD;
  println!("{}", ans);
}
