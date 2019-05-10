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
    aa: [usize; n]
  }
  let mut dp = vec![vec![vec![0.0; n+1]; n+1]; n+1];
  let mut pop = vec![0usize; 4];
  for &a in aa.iter() {
    pop[a] += 1;
  }
  let total_sushi: usize = aa.iter().sum();
  dp[pop[1]][pop[2]][pop[3]] = 1.0;
  for three in (0..n+1).rev() {
    for two in (0..n+1).rev() {
      for one in (0..n+1).rev() {
        let dish = (one + two + three) as f64;
        if three > 0 && two + 1 <= n {
          dp[one][two+1][three-1] += dp[one][two][three] * (three as f64) / dish;
        }
        if two > 0 && one + 1 <= n {
          dp[one+1][two-1][three] += dp[one][two][three] * (two as f64) / dish;
        }
        if one > 0 {
          dp[one-1][two][three] += dp[one][two][three] * (one as f64) / dish;
        }
      }
    }
  }
  let mut ans = 0.0;
  for sushi in (1..total_sushi+1).rev() {
    for three in 0..(sushi / 3 + 1) {
      for two in 0..((sushi - three * 3) / 2 + 1) {
        let one = sushi - three * 3 - two * 2;
        if two > n || one > n {
          continue;
        }
        ans += dp[one][two][three] * n as f64 / (one + two + three) as f64;
      }
    }
  }
  println!("{:.15}", ans);
}
