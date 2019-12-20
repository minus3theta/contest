#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

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

  ($iter:expr, [ $t:tt ]) => {{
    let len = read_value!($iter, usize);
    (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  }};

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

const INF: i64 = 1 << 60;

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    k: usize,
    hs: [i64; n],
  }
  let mut vs = hs.clone();
  vs.push(0);
  vs.sort();
  vs.dedup();
  let mut dp = vec![vec![vec![INF; vs.len()]; k + 1]; n + 1];
  for cc in 0 .. k + 1 {
    dp[0][cc][0] = 0;
  }
  for (i, &h) in hs.iter().enumerate() {
    for cc in 0 .. k + 1 {
      let mut acc = INF;
      for (j, &v) in vs.iter().enumerate() {
        acc = cmp::min(acc, dp[i][cc][j] - v);
        if v == h {
          dp[i+1][cc][j] = cmp::min(dp[i+1][cc][j], acc + v);
        } else if cc + 1 <= k {
          dp[i+1][cc+1][j] = cmp::min(dp[i+1][cc+1][j], acc + v);
        }
      }
      let mut acc = INF;
      for (j, &v) in vs.iter().enumerate().rev() {
        acc = cmp::min(acc, dp[i][cc][j]);
        if v == h {
          dp[i+1][cc][j] = cmp::min(dp[i+1][cc][j], acc);
        } else if cc + 1 <= k {
          dp[i+1][cc+1][j] = cmp::min(dp[i+1][cc+1][j], acc);
        }
      }
    }
  }

  let mut ans = INF;
  for cc in 0 .. k + 1 {
    for j in 0 .. vs.len() {
      ans = cmp::min(ans, dp[n][cc][j]);
    }
  }
  puts!("{}\n", ans);
}