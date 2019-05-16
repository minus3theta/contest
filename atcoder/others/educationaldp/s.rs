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

fn main() {
  input! {
    k: chars,
    d: usize
  }
  let n = k.len();
  let mut same = vec![vec![0i64; d]; 2];
  let mut less = vec![vec![0i64; d]; 2];
  let k: Vec<_> = k.into_iter().map(|x| x.to_digit(10).unwrap() as usize).collect();
  same[0][0] = 1;
  for (i, &x) in k.iter().enumerate() {
    for rem in 0 .. d {
      same[(i + 1) % 2][rem] = 0;
      less[(i + 1) % 2][rem] = 0;
    }
    for rem in 0 .. d {
      same[(i + 1) % 2][(rem + x) % d] += same[i % 2][rem];
      same[(i + 1) % 2][(rem + x) % d] %= MOD;
      for digit in 0 .. 10 {
        if digit < x {
          less[(i + 1) % 2][(rem + digit) % d] += same[i % 2][rem];
        }
        less[(i + 1) % 2][(rem + digit) % d] += less[i % 2][rem];
        less[(i + 1) % 2][(rem + digit) % d] %= MOD;
      }
    }
  }
  let ans = (same[n % 2][0] + less[n % 2][0] + MOD - 1) % MOD;
  println!("{}", ans);
}
