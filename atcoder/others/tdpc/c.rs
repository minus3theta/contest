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
    k: usize,
    r: [f64; 1 << k],
  }
  let n = 1 << k;
  let mut win = vec![vec![0.0; n]; n];
  for p in 0 .. n {
    for q in 0 .. n {
      win[p][q] = 1.0 / (1.0 + 10.0f64.powf((r[q] - r[p]) / 400.0));
    }
  }
  let mut dp = vec![vec![0.0; n]; k + 1];
  for x in dp[0].iter_mut() {
    *x = 1.0;
  }
  for l in 0 .. k {
    for p in 0 .. n {
      let p_block = p / (1 << l);
      let q_block = if p_block % 2 == 0 { p_block + 1 } else { p_block - 1 };
      for q in q_block * (1 << l) .. (q_block + 1) * (1 << l) {
        dp[l+1][p] += dp[l][q] * win[p][q];
      }
      dp[l+1][p] *= dp[l][p];
    }
  }

  for &v in dp[k].iter() {
    println!("{:.10}", v);
  }
}
