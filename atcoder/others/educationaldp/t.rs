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
    n: usize,
    s: chars
  }
  let mut dp = vec![vec![0i64; n]; n];
  dp[0][0] = 1;
  for (pos, &c) in s.iter().enumerate() {
    let len = pos + 1;
    match c {
      '<' => {
        for tail in 1 .. len + 1 {
          dp[len][tail] = (dp[len][tail - 1] + dp[len - 1][tail - 1]) % MOD;
        }
      },
      '>' => {
        for tail in (0 .. len).rev() {
          dp[len][tail] = (dp[len][tail + 1] + dp[len - 1][tail]) % MOD;
        }
      },
      _ => unreachable!()
    }
  }
  let ans = dp[n-1].iter().sum::<i64>() % MOD;
  println!("{}", ans);
}
