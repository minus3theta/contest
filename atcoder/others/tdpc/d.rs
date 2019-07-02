#[allow(unused_imports)]
use std::cmp;

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

fn div(d: &mut i64, div: i64) -> usize {
  let mut count = 0;
  while *d % div == 0 {
    *d /= div;
    count += 1;
  }
  count
}

fn main() {
  input! {
    n: usize,
    d: i64,
  }
  let mut d = d;
  let d2 = div(&mut d, 2);
  let d3 = div(&mut d, 3);
  let d5 = div(&mut d, 5);
  if d != 1 {
    println!("{:.15}", 0.0);
    return;
  }
  let mut dp = vec![vec![vec![vec![0.0; d5 + 1]; d3 + 1]; d2 + 1]; n + 1];
  dp[0][0][0][0] = 1.0;
  for c in 0 .. n {
    for c2 in 0 .. d2 + 1 {
      for c3 in 0 .. d3 + 1 {
        for c5 in 0 .. d5 + 1 {
          // 1
          dp[c+1][c2][c3][c5] += dp[c][c2][c3][c5] / 6.0;
          // 2
          dp[c+1][cmp::min(c2 + 1, d2)][c3][c5] += dp[c][c2][c3][c5] / 6.0;
          // 3
          dp[c+1][c2][cmp::min(c3 + 1, d3)][c5] += dp[c][c2][c3][c5] / 6.0;
          // 4
          dp[c+1][cmp::min(c2 + 2, d2)][c3][c5] += dp[c][c2][c3][c5] / 6.0;
          // 5
          dp[c+1][c2][c3][cmp::min(c5 + 1, d5)] += dp[c][c2][c3][c5] / 6.0;
          // 6
          dp[c+1][cmp::min(c2 + 1, d2)][cmp::min(c3 + 1, d3)][c5] += dp[c][c2][c3][c5] / 6.0;
        }
      }
    }
  }
  println!("{:.15}", dp[n][d2][d3][d5]);
}
