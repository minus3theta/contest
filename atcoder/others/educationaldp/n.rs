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

fn main() {
  input! {
    n: usize,
    aa: [i64; n]
  }
  let mut dp = vec![vec![1e18 as i64; n+1]; n+1];
  let mut sum = vec![0i64; n+1];
  for (i, &a) in aa.iter().enumerate() {
    sum[i+1] = sum[i] + a;
  }
  for len in 0 .. n + 1 {
    for l in 0 .. n + 1 - len {
      let r = l + len;
      if len <= 1 {
        dp[l][r] = 0;
        continue;
      }
      let size = sum[r] - sum[l];
      for m in l .. r {
        dp[l][r] = cmp::min(dp[l][r], dp[l][m] + dp[m][r] + size);
      }
    }
  }
  println!("{}", dp[0][n]);
}
