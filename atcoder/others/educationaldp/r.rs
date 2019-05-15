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

fn mat_mul(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
  let n = a.len();
  let mut x = vec![vec![0i64; n]; n];
  for i in 0 .. n {
    for j in 0 .. n {
      for k in 0 .. n {
        x[i][j] += a[i][k] * b[k][j];
        x[i][j] %= MOD;
      }
    }
  }
  x
}

fn mat_pow(a: &Vec<Vec<i64>>, e: i64) -> Vec<Vec<i64>> {
  if e == 1 {
    a.clone()
  } else if e % 2 == 0 {
    let x = mat_pow(a, e / 2);
    mat_mul(&x, &x)
  } else {
    let x = mat_pow(a, e - 1);
    mat_mul(&x, &a)
  }
}

fn main() {
  input! {
    n: usize,
    k: i64,
    a: [[i64; n]; n]
  }
  let x = mat_pow(&a, k);
  let sum: i64 = x.iter().map::<i64, _>(|v| v.iter().sum()).sum();
  println!("{}", sum % MOD);
}
